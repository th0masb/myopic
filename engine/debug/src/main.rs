use clap::Parser;
use myopic_brain::{Evaluator, SearchParameters};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    pgn: String,
    #[arg(long)]
    depth: usize,
    #[arg(long)]
    table_size: usize,
}

fn main() {
    let args = Args::parse();
    let mut state = Evaluator::default();
    state.play_pgn(args.pgn.as_str()).unwrap();
    if args.depth == 0 {
        println!("Static: {}", state.relative_eval());
        println!("Quiescent: {}", myopic_brain::quiescent::full_search(&mut state).unwrap());
    } else {
        let outcome = myopic_brain::search(
            state,
            SearchParameters { terminator: args.depth, table_size: args.table_size },
        )
        .unwrap();
        println!("{}", serde_json::to_string_pretty(&outcome).unwrap());
    }
}
