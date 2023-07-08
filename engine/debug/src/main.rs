use std::str::FromStr;
use clap::{Parser, Subcommand};
use myopic_brain::{Evaluator, SearchParameters, TranspositionsImpl};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SearchPgn {
        #[arg(long)]
        pgn: String,
        #[arg(long)]
        depth: usize,
        #[arg(long)]
        table_size: usize,
    },
    SearchFen {
        #[arg(long)]
        fen: String,
        #[arg(long)]
        depth: usize,
        #[arg(long)]
        table_size: usize,
    },
}

fn main() {
    match Cli::parse().command {
        Commands::SearchPgn { pgn, depth, table_size } => {
            let mut state = Evaluator::default();
            state.play_pgn(pgn.as_str()).unwrap();
            run_search(state, depth, table_size);
        }
        Commands::SearchFen { fen, depth, table_size } => {
            let state = Evaluator::from_str(fen.as_str()).unwrap();
            run_search(state, depth, table_size);
        }
    }
}

fn run_search(mut state: Evaluator, depth: usize, table_size: usize) {
    if depth == 0 {
        println!("Static: {}", state.relative_eval());
        println!("Quiescent: {}", myopic_brain::quiescent::full_search(&mut state).unwrap());
    } else {
        let outcome = myopic_brain::search(
            state,
            SearchParameters {
                terminator: depth,
                table: &mut TranspositionsImpl::new(table_size)
            },
        );
        println!("{}", serde_json::to_string_pretty(&outcome.unwrap()).unwrap());
    }
}
