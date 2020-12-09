mod data;
mod errors;
mod game_stream;

use chrono::Utc;
use errors::Errors;
use game_stream::GameStream;
use myopic_board::{FenComponent, Move, MutBoard};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::thread;
use std::{env, fs};
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
    /// The depth we will search into games.
    #[structopt(short = "d", long = "search-depth")]
    #[serde(rename = "search-depth")]
    search_depth: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();

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
                Ok(game) => match parse_entries(opt.search_depth, game.as_str()) {
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
        println!("{}", serde_json::to_string(&database_entry)?);
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
    path.to_str().expect("Couldn't convert path to string").to_string()
}

fn parse_entries(depth: usize, game: &str) -> Result<Vec<CollectionEntry>, errors::Error> {
    let moves: Vec<Move> = myopic_board::parse::pgn(game)
        .map_err(|msg| errors::err(msg.as_str()))?
        .into_iter()
        .take(depth)
        .collect();

    let (mut board, mut entries) = (myopic_board::start_position(), vec![]);
    for mv in moves {
        match mv {
            Move::Enpassant(_, _) => {
                println!("ENPASSANT: Ignoring enpassant move in {}", game);
            }
            _ => {
                entries.push(CollectionEntry {
                    position: board.to_partial_fen(&[
                        FenComponent::Board,
                        FenComponent::Active,
                        FenComponent::CastlingRights,
                    ]),
                    mv: mv.uci_format(),
                });
            }
        }
        board.evolve(&mv);
    }

    Ok(entries)
}

#[derive(Debug, Serialize)]
struct CollectionEntry {
    position: String,
    mv: String,
}
