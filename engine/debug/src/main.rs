use clap::Parser;
use myopic_brain::{Board, Evaluator, SearchParameters};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    pgn: String,
    #[arg(long)]
    position: String,
    #[arg(long)]
    depth: usize,
    #[arg(long)]
    table_size: usize,
}

fn main() {
    let args = Args::parse();
    let mut state = Evaluator::from(create_board(&args));
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

fn create_board(args: &Args) -> Board {
    if args.pgn.is_empty() {
        args.position.parse().unwrap()
    } else {
        let mut board = Board::default();
        board.play_pgn(args.pgn.as_str()).unwrap();
        board
    }
}
