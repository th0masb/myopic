use clap::{Parser, Subcommand};
use myopic_brain::{Board, Evaluator, FenPart, Moves, SearchParameters, Transpositions, TreeNode};
use std::str::FromStr;

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
    Moves {
        #[arg(long)]
        fen: String,
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
        Commands::Moves { fen } => {
            let board = fen.as_str().parse::<Board>().unwrap();
            let moves: Vec<_> =
                board.moves(Moves::All).into_iter().map(|m| m.uci_format()).collect();
            println!("{}", serde_json::to_string_pretty(&moves).unwrap());
        }
    }
}

struct DebugTranspositions {
    store: Vec<Option<(String, TreeNode)>>,
}

impl DebugTranspositions {
    pub fn new(size: usize) -> DebugTranspositions {
        DebugTranspositions { store: vec![None; size] }
    }
}

const FEN_PARTS: [FenPart; 4] =
    [FenPart::Board, FenPart::Active, FenPart::CastlingRights, FenPart::Enpassant];

impl Transpositions for DebugTranspositions {
    fn get(&self, pos: &Board) -> Option<&TreeNode> {
        let hash = pos.hash();
        let index = (hash % self.store.len() as u64) as usize;
        if let Some((existing, n)) = self.store[index].as_ref() {
            if n.matches(hash) {
                let new_pos = pos.to_fen_parts(&FEN_PARTS);
                if existing.as_str() != new_pos.as_str() {
                    panic!("Collision: {} <-> {}", existing, new_pos)
                }
                Some(n)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn put(&mut self, pos: &Board, n: TreeNode) {
        let hash = pos.hash();
        let index = (hash % self.store.len() as u64) as usize;
        if !pos.moves(Moves::All).contains(&n.get_move()) {
            panic!("Bad node {} <-> {:?}", pos.to_fen(), n)
        }
        self.store[index] = Some((pos.to_fen_parts(&FEN_PARTS), n))
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
                table: &mut DebugTranspositions::new(table_size),
            },
        );
        println!("{}", serde_json::to_string_pretty(&outcome.unwrap()).unwrap());
    }
}
