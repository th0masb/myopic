mod data;
mod errors;
mod game_stream;

use errors::Errors;
use game_stream::GameStream;
use myopic_board::{FenPart, Move, ChessBoard};
use std::{collections::HashMap, error::Error, fs, fs::File, path::PathBuf};
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, StructOpt, Serialize)]
#[structopt(name = "openings-generator")]
struct Opt {
    /// The source directory containing the pgn files to extract
    /// positions from.
    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,
    /// The depth we will search into games starting
    /// from the offset.
    #[structopt(short = "d", long = "depth")]
    #[serde(rename = "depth")]
    search_depth: usize,
    /// The depth we will start the search into a game.
    #[structopt(short = "o", long = "offset", default_value = "0")]
    #[serde(rename = "offset")]
    search_offset: usize,
    /// FEN components to include and their ordering
    #[structopt(long = "format", default_value = "bac")]
    #[serde(rename = "format")]
    fen_format: String,
    /// Only the positions will be output.
    #[structopt(long = "positions-only")]
    #[serde(rename = "positions-only")]
    positions_only: bool,
}

fn parse_fen_components(input: &str) -> Vec<FenPart> {
    input
        .chars()
        .flat_map(|c| match c {
            'b' => vec![FenPart::Board],
            'a' => vec![FenPart::Active],
            'c' => vec![FenPart::CastlingRights],
            'e' => vec![FenPart::Enpassant],
            'h' => vec![FenPart::HalfMoveCount],
            'm' => vec![FenPart::MoveCount],
            _ => vec![],
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    let fen_components = parse_fen_components(opt.fen_format.as_str());

    eprintln!("{}", chrono::Utc::now());
    eprintln!();
    eprintln!("Starting DynamoDB data generator with parameters:");
    eprintln!("{}", serde_json::to_string_pretty(&opt)?);
    eprintln!();

    let file_paths = get_pgn_file_paths(&opt)?;
    let game_counts = count_games(&file_paths)?;

    let mut store = data::PositionStore::default();

    let game_progress = indicatif::ProgressBar::new(0);
    game_progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} extracting: {msg}")
            .progress_chars("##-"),
    );

    let mut errors = Errors::default();
    let n_files = file_paths.len();
    for (i, path) in get_pgn_file_paths(&opt)?.into_iter().enumerate() {
        // Update progress
        let file_name = path_to_string(&path);
        game_progress.set_message(&format!("({}/{})[{}]", i + 1, n_files, file_name.as_str()));
        game_progress.set_length(*game_counts.get(&file_name).unwrap() as u64);
        game_progress.reset();

        let pgn_file = File::open(&path)?;

        for (i, game_result) in GameStream::new(pgn_file).enumerate() {
            game_progress.set_position(i as u64);
            match game_result {
                Err(_) => {
                    errors.add_read_error(file_name.clone(), i);
                }
                Ok(game) => match parse_entries(
                    &fen_components,
                    opt.search_offset,
                    opt.search_depth,
                    game.as_str(),
                ) {
                    Err(_) => {
                        errors.add_parse_error(file_name.clone(), i);
                    }
                    Ok(entries) => entries
                        .into_iter()
                        .for_each(|entry| store.process(entry.position, entry.mv)),
                },
            }
        }
    }

    game_progress.finish();
    eprintln!();
    for database_entry in store.entries() {
        if opt.positions_only {
            println!("{}", database_entry.position)
        } else {
            println!("{}", serde_json::to_string(&database_entry)?);
        }
    }
    eprintln!("Errors:\n{}\n", serde_json::to_string_pretty(&errors)?);
    eprintln!("Stats:\n{}", serde_json::to_string_pretty(store.stats())?);

    Ok(())
}

fn get_pgn_file_paths(opt: &Opt) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut dest = Vec::new();
    for entry in fs::read_dir(&opt.source)? {
        dest.push(entry?.path())
    }
    Ok(dest)
}

fn count_games(file_paths: &Vec<PathBuf>) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let mut dest = HashMap::new();
    for path in file_paths {
        let path_str = path_to_string(path);
        dest.insert(path_str, GameStream::new(File::open(path)?).count());
    }
    Ok(dest)
}

fn path_to_string(path: &PathBuf) -> String {
    path.to_str()
        .expect("Couldn't convert path to string")
        .to_string()
}

fn parse_entries(
    format: &Vec<FenPart>,
    offset: usize,
    depth: usize,
    game: &str,
) -> Result<Vec<CollectionEntry>, anyhow::Error> {
    let mut board = myopic_board::start();
    let moves: Vec<Move> = board.play_pgn(game)?
        .into_iter()
        .take(offset + depth)
        .collect();

    let (mut board, mut entries) = (myopic_board::start(), vec![]);
    for (i, mv) in moves.into_iter().enumerate() {
        if i >= offset {
            match mv {
                // Ignore enpassant moves for now
                Move::Enpassant { .. } => {}
                _ => {
                    entries.push(CollectionEntry {
                        position: board.to_fen_parts(format.as_slice()),
                        mv: mv.uci_format(),
                    });
                }
            }
        }
        board.make(mv)?;
    }

    Ok(entries)
}

#[derive(Debug, Serialize)]
struct CollectionEntry {
    position: String,
    mv: String,
}
