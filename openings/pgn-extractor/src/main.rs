mod data;
mod errors;
mod game_stream;
mod position;

use anyhow::{Result};
use errors::Errors;
use game_stream::GameStream;
use itertools::Itertools;
use myopic_board::{ChessBoard, Move};
use serde_derive::{Serialize};

use std::{collections::HashMap, fs, fs::File, path::PathBuf};
use structopt::StructOpt;
use position::PositionFormat;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, StructOpt)]
#[structopt(name = "position-extractor")]
struct Opt {
    /// The source directory containing the pgn files to extract
    /// positions from.
    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,
    /// The depth we will search into games starting
    /// from the offset.
    #[structopt(short = "d", long = "depth")]
    search_depth: usize,
    /// The depth we will start the search into a game.
    #[structopt(short = "o", long = "offset", default_value = "0")]
    search_offset: usize,
    /// FEN components to include and their ordering
    #[structopt(long = "format")]
    position_format: PositionFormat,
    /// Only the positions will be output.
    #[structopt(long = "positions-only")]
    positions_only: bool,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    eprintln!("{}", chrono::Utc::now());
    eprintln!("Starting position extractor");
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
                    &opt.position_format,
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

fn get_pgn_file_paths(opt: &Opt) -> Result<Vec<PathBuf>> {
    let mut dest = Vec::new();
    for entry in fs::read_dir(&opt.source)? {
        dest.push(entry?.path())
    }
    Ok(dest)
}

fn count_games(file_paths: &Vec<PathBuf>) -> Result<HashMap<String, usize>> {
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

fn gen_moves(offset: usize, depth: usize, game: &str) -> Result<Vec<Move>> {
    let mut board = myopic_board::start();
    Ok(board
        .play_pgn(game)?
        .into_iter()
        .take(offset + depth)
        .collect())
}

fn parse_entries(
    format: &PositionFormat,
    offset: usize,
    depth: usize,
    game: &str,
) -> Result<Vec<CollectionEntry>> {
    let moves = gen_moves(offset, depth, game)?;
    let (mut board, mut entries) = (myopic_board::start(), vec![]);
    for (i, mv) in moves.into_iter().enumerate() {
        if i >= offset {
            match mv {
                // Ignore enpassant moves for now
                Move::Enpassant { .. } => {}
                _ => {
                    entries.push(CollectionEntry {
                        mv: mv.uci_format(),
                        position: match format {
                            PositionFormat::UciSequence => board
                                .previous_moves()
                                .into_iter()
                                .map(|m| m.uci_format())
                                .join(" "),
                            PositionFormat::Fen { format } => {
                                board.to_partial_fen(format.0.as_slice())
                            }
                        },
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
