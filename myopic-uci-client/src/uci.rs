use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use regex::{Match, Regex};

use crate::{board, search};
use crate::base::square::Square;
use crate::base::StrResult;
use crate::board::{Board, BoardImpl, Move, MoveComputeType};
use crate::eval::{EvalBoard, SimpleEvalBoard};
use crate::parse::patterns;
use crate::pieces::Piece;
use crate::search::{SearchCmdTx, SearchCommand, SearchResult};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Uninitialized,
    Configuring,
    WaitingForPosition,
    WaitingForGo,
    Searching,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Input {
    WhatState,
    Uci,
    IsReady,
    UciNewGame,
    Stop,
    Quit,
    Position(String, Vec<String>),
    Go(Vec<GoCommand>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum GoCommand {
    Depth(usize),
    MoveTime(usize),
    WhiteTime(usize),
    BlackTime(usize),
    WhiteInc(usize),
    BlackInc(usize),
    Infinite,
}

const ENGINE_NAME: &'static str = "Myopic";
const ENGINE_AUTHOR: &'static str = "Thomas Ball";
const WHAT_STATE: &'static str = "whatstate";
const READYOK: &'static str = "readyok";
const BESTMOVE: &'static str = "bestmove";
const ISREADY: &'static str = "isready";
const QUIT: &'static str = "quit";
const STOP: &'static str = "stop";
const UCI: &'static str = "uci";
const UCIOK: &'static str = "uciok";
const UCINEWGAME: &'static str = "ucinewgame";
const POSITION: &'static str = "position";
const GO: &'static str = "go";


const SLEEP_TIME: Duration = Duration::from_millis(100);
type DefaultBoard = SimpleEvalBoard<BoardImpl>;

pub fn uci_main() -> () {
    // Engine input command channel
    let cmd_input_rx = initialize_input_thread();
    let (search_input_tx, search_output_rx) = search::init::<DefaultBoard>();
    // Begin the main control loop
    let mut engine_state = State::Uninitialized;
    loop {
        thread::sleep(SLEEP_TIME);
        // If currently in a search state then check if a best move has been computed,
        // if it has then output the result and update the engine state.
        if engine_state == State::Searching {
            // Don't block for the result here
            match search_output_rx.try_recv() {
                Err(_) => (),
                Ok(result) => engine_state = complete_search(result, &search_input_tx),
            }
        }

        // Check for a new input and process the command if it is present.
        match cmd_input_rx.try_recv() {
            Err(_) => continue,
            Ok(cmd) => match (engine_state, cmd) {
                // In any state if we get a quit command then break.
                (_, Input::Quit) => break,
                // Debug command to print the current engine state
                (_, Input::WhatState) => println!("{:?}", engine_state),
                // Procedure from an uninitialized state
                (State::Uninitialized, Input::Uci) => {
                    engine_state = State::Configuring;
                    initialize();
                }

                // Procedure from the config state, not complete yet
                // since we don't actually support any config.
                (State::Configuring, Input::IsReady) => {
                    engine_state = State::WaitingForPosition;
                    println!("{}", READYOK)
                }

                // Procedure from the positional setup state.
                (State::WaitingForPosition, Input::UciNewGame) => (),
                (State::WaitingForPosition, Input::Position(fen, moves)) => {
                    engine_state = update_position(fen, moves, &search_input_tx);
                }

                // Procedure from the pre-searching state.
                (State::WaitingForGo, Input::Go(commands)) => {
                    for cmd in convert_go_setup_commands(commands) {
                        search_input_tx.send(cmd).unwrap();
                    }
                    engine_state = State::Searching;
                    search_input_tx.send(SearchCommand::Go).unwrap();
                    println!("Beginning search");
                }

                // Procedure from the searching state.
                (State::Searching, Input::Stop) => {
                    // block for the result
                    search_input_tx.send(SearchCommand::Stop).unwrap();
                    let result = search_output_rx.recv().unwrap();
                    engine_state = complete_search(result, &search_input_tx);
                }

                (_, Input::IsReady) => println!("{}", READYOK),
                // Otherwise do nothing
                _ => (),
            },
        }
    }
}

fn complete_search<B: EvalBoard>(result: SearchResult, tx: &SearchCmdTx<B>) -> State {
    match result {
        Err(_) => State::Searching,
        Ok(details) => {
            // Print best move if there is one.
            println!("{} {}", BESTMOVE, format_move(details.best_move));
            // Reset the search timing
            tx.send(SearchCommand::Infinite).unwrap();
            // Return
            State::WaitingForPosition
        }
    }
}

fn update_position(fen: String, moves: Vec<String>, tx: &SearchCmdTx<DefaultBoard>) -> State {
    match crate::eval::new_board(&fen) {
        Err(_) => State::WaitingForPosition,
        Ok(mut board) => {
            let mut parsed_correctly = true;
            for mv in moves {
                match parse_long_algebraic_move(&mut board, &mv) {
                    Err(_) => parsed_correctly = false,
                    Ok(parsed_move) => {
                        board.evolve(&parsed_move);
                        ()
                    }
                }
            }
            if parsed_correctly {
                tx.send(SearchCommand::Root(board)).unwrap();
                State::WaitingForGo
            } else {
                State::WaitingForPosition
            }
        }
    }
}

fn convert_go_setup_commands<B: EvalBoard>(commands: Vec<GoCommand>) -> Vec<SearchCommand<B>> {
    let (mut infinite, mut max_depth, mut max_time) = (false, -1, -1);
    let (mut w_base, mut w_inc, mut b_base, mut b_inc) = (0, 0, 0, 0);
    for command in commands {
        match command {
            GoCommand::WhiteTime(time) => w_base = time,
            GoCommand::WhiteInc(time) => w_inc = time,
            GoCommand::BlackTime(time) => b_base = time,
            GoCommand::BlackInc(time) => b_inc = time,
            GoCommand::Infinite => infinite = true,
            GoCommand::Depth(depth) => max_depth = depth as i32,
            GoCommand::MoveTime(time) => max_time = time as i32,
        }
    }
    if infinite {
        vec![SearchCommand::Infinite]
    } else if max_depth > 0 {
        vec![SearchCommand::Depth(max_depth as usize)]
    } else if max_time > 0 {
        vec![SearchCommand::Time(max_time as usize)]
    } else {
        vec![SearchCommand::GameTime { w_base, w_inc, b_base, b_inc }]
    }
}

fn parse_long_algebraic_move<B: Board>(board: &mut B, mv: &String) -> StrResult<Move> {
    if mv.len() < 4 || mv.len() > 5 {
        return Err(format!("Illegal length: {}", mv.len()));
    }
    let source = Square::from_string(&mv.chars().take(2).collect::<String>())?;
    let target = Square::from_string(&mv.chars().skip(2).take(2).collect::<String>())?;
    let promote = mv.chars().nth(4).map(|c| c.to_string());
    board
        .compute_moves(MoveComputeType::All)
        .into_iter()
        .find(|mv| match mv {
            &Move::Standard(_, s, t) => source == s && target == t,
            &Move::Enpassant(s, t) => source == s && target == t,
            &Move::Promotion(s, t, p) => {
                source == s && target == t && Some(format_piece(p).to_string()) == promote
            }
            &Move::Castle(zone) => {
                let (_, ks, kt) = zone.king_data();
                source == ks && target == kt
            }
        })
        .ok_or(format!("No moves matching {}", mv.clone()))
}

fn format_move(input: Move) -> String {
    let mut dest = String::new();
    let (source, target, promotion) = match input {
        Move::Standard(_, s, t) => (s, t, None),
        Move::Promotion(s, t, p) => (s, t, Some(p)),
        Move::Enpassant(s, t) => (s, t, None),
        Move::Castle(zone) => {
            let (_, s, t) = zone.king_data();
            (s, t, None)
        }
    };
    dest.push_str(format!("{}", source).to_lowercase().as_str());
    dest.push_str(format!("{}", target).to_lowercase().as_str());
    promotion.map(|piece: Piece| dest.push_str(format_piece(piece)));
    dest
}

fn format_piece(piece: Piece) -> &'static str {
    match piece {
        Piece::WQ | Piece::BQ => "q",
        Piece::WR | Piece::BR => "r",
        Piece::WB | Piece::BB => "b",
        Piece::WN | Piece::BN => "n",
        _ => panic!(),
    }
}

fn initialize() {
    println!("id name {}", ENGINE_NAME);
    println!("id author {}", ENGINE_AUTHOR);
    println!("{}", UCIOK);
}

/// Spawn a user input thread, it simply listens for
/// standard input, parses the string to an engine command
/// and transmits the result (if valid) along the given
/// sender instance.
fn initialize_input_thread() -> Receiver<Input> {
    let (cmd_input_tx, cmd_input_rx) = mpsc::channel::<Input>();
    thread::spawn(move || loop {
        let mut dest = String::new();
        match io::stdin().read_line(&mut dest) {
            Ok(_) => (),
            Err(_) => continue,
        }
        let cmd = parse_engine_command(dest.trim().to_lowercase().to_owned());
        if cmd.is_some() {
            match cmd_input_tx.send(cmd.unwrap()) {
                _ => (),
            }
        }
    });
    cmd_input_rx
}

fn parse_engine_command(content: String) -> Option<Input> {
    match content.as_str() {
        WHAT_STATE => Some(Input::WhatState),
        UCI => Some(Input::Uci),
        ISREADY => Some(Input::IsReady),
        UCINEWGAME => Some(Input::UciNewGame),
        STOP => Some(Input::Stop),
        QUIT => Some(Input::Quit),
        x => {
            if x.starts_with(POSITION) {
                parse_position_command(content)
            } else if x.starts_with(GO) {
                Some(Input::Go(parse_go_command(content)))
            } else {
                None
            }
        }
    }
}

fn parse_go_command(content: String) -> Vec<GoCommand> {
    lazy_static! {
            static ref INFINITE: Regex = re("infinite".to_owned());
            static ref PONDER: Regex = re("ponder".to_owned());
            static ref DEPTH: Regex = re(format!("depth {}", patterns::int().as_str()));
            static ref MOVETIME: Regex = re(format!("movetime {}", patterns::int().as_str()));
            static ref WHITETIME: Regex = re(format!("wtime {}", patterns::int().as_str()));
            static ref BLACKTIME: Regex = re(format!("btime {}", patterns::int().as_str()));
            static ref WHITEINC: Regex = re(format!("winc {}", patterns::int().as_str()));
            static ref BLACKINC: Regex = re(format!("binc {}", patterns::int().as_str()));
        }
    let content_ref = content.as_str();
    let extract =
        |m: Match| patterns::int().find(m.as_str()).unwrap().as_str().parse::<usize>().unwrap();
    let mut dest = Vec::new();
    &INFINITE.find(content_ref).map(|_| dest.push(GoCommand::Infinite));
    &DEPTH.find(content_ref).map(|m| dest.push(GoCommand::Depth(extract(m))));
    &MOVETIME.find(content_ref).map(|m| dest.push(GoCommand::MoveTime(extract(m))));
    &WHITETIME.find(content_ref).map(|m| dest.push(GoCommand::WhiteTime(extract(m))));
    &BLACKTIME.find(content_ref).map(|m| dest.push(GoCommand::BlackTime(extract(m))));
    &WHITEINC.find(content_ref).map(|m| dest.push(GoCommand::WhiteInc(extract(m))));
    &BLACKINC.find(content_ref).map(|m| dest.push(GoCommand::BlackInc(extract(m))));
    dest
}

fn parse_position_command(content: String) -> Option<Input> {
    let c_ref = content.as_str();
    let moves = la_move().find_iter(c_ref).map(|m| m.as_str().to_owned()).collect();
    position().find(c_ref).map(|m| match m.as_str() {
        "startpos" => Input::Position(board::START_FEN.to_owned(), moves),
        x => Input::Position(x.to_owned(), moves),
    })
}

#[cfg(test)]
mod test {
    use crate::board;
    use crate::uci::Input;

    #[test]
    fn test_parse_position() {
        let ppc = super::parse_position_command;
        assert_eq!(
            Some(Input::Position(board::START_FEN.to_owned(), vec!["e2e4".to_owned()])),
            ppc("position startpos moves e2e4".to_owned())
        );
    }
}

fn position() -> &'static Regex {
    lazy_static! {
        static ref PE: Regex = re(format!("(startpos|{})", patterns::fen().as_str()));
    }
    &PE
}

fn la_move() -> &'static Regex {
    lazy_static! {
        static ref MOVE: Regex = re(r"([a-h][1-8]){2}[qrnb]?".to_owned());
    }
    &MOVE
}

fn re(pattern: String) -> Regex {
    Regex::new(pattern.as_str()).unwrap()
}
