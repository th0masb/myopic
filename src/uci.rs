use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use regex::{Match, Regex};

use crate::board::{BoardImpl, Move};
use crate::board::Move::Standard;
use crate::eval::SimpleEvalBoard;
use crate::pieces::Piece;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Uninitialized,
    Configuring,
    WaitingForPosition,
    WaitingForGo,
    Searching,
    Pondering,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Input {
    Uci,
    IsReady,
    UciNewGame,
    Stop,
    PonderHit,
    Quit,
    Position(String, Vec<String>),
    Go(Vec<GoCommand>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum GoCommand {
    SearchMoves(Vec<String>),
    Depth(usize),
    MoveTime(usize),
    WhiteTime(usize),
    BlackTime(usize),
    WhiteInc(usize),
    BlackInc(usize),
    Ponder,
    Infinite,
}

const ENGINE_NAME: &'static str = "Myopic";
const ENGINE_AUTHOR: &'static str = "Thomas Ball";

pub fn uci_main() -> () {
    // Engine input command channel
    let cmd_input_rx = initialize_input_thread();
    let (search_input_rx, search_output_tx) = crate::search::init::<SimpleEvalBoard<BoardImpl>>();
    // Begin the main control loop
    let mut engine_state = State::Uninitialized;
    loop {
        // If currently in a search state then check if a best move has been computed,
        // if it has then output the result and update the engine state.
        if engine_state == State::Searching {
            match search_output_tx.try_recv() {
                Err(_) => (),
                Ok(result) => match result {
                    Err(_) => engine_state = State::WaitingForPosition,
                    Ok(details) => {
                        unimplemented!()
                    }
                }
            }
        }

        // Check for a new input and process the command if it is present.
        match cmd_input_rx.try_recv() {
            Err(_) => continue,
            Ok(cmd) => match (engine_state, cmd) {
                // In any state if we get a quit command then break.
                (_, Input::Quit) => break,
                // Procedure from an uninitialized state
                (State::Uninitialized, Input::Uci) => {
                    engine_state = State::Configuring;
                    initialize();
                }

                // Procedure from the config state, not complete yet
                // since we don't actually support any config.
                (State::Configuring, Input::IsReady) => {
                    engine_state = State::WaitingForPosition;
                    println!("readyok")
                }

                // Procedure from the positional setup state.
                (State::WaitingForPosition, Input::UciNewGame) => (),
                (State::WaitingForPosition, Input::Position(fen, moves)) => {
                    unimplemented!()
//                    if searcher.setup_position(fen, moves) {
//                        engine_state = State::WaitingForGo;
//                    }
                }

                (_, Input::IsReady) => println!("readyok"),

                _ => (),
            },
        }
    }
}

fn format_move(input: Move) -> String {
    let mut dest = String::new();
    let (source, target, promotion) = match input {
        Move::Standard(p, s, t) => (s, t, None),
        Move::Promotion(s, t, p) => (s, t, Some(p)),
        _ => unimplemented!()
    };
    dest.push_str(format!("{}", source).as_str());
    dest.push_str(format!("{}", target).as_str());
    promotion.map(|piece: Piece| dest.push_str(format_piece(piece)));
    dest
}

fn format_piece(piece: Piece) -> &'static str {
    match piece {
        Piece::WQ | Piece::BQ => "q",
        Piece::WR | Piece::BR => "r",
        Piece::WB | Piece::BB => "b",
        Piece::WN | Piece::BN => "n",
        _ => panic!()
    }
}

fn initialize() {
    println!("id name {}", ENGINE_NAME);
    println!("id author {}", ENGINE_AUTHOR);
    println!("uciok");
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
        "uci" => Some(Input::Uci),
        "isready" => Some(Input::IsReady),
        "ucinewgame" => Some(Input::UciNewGame),
        "stop" => Some(Input::Stop),
        "ponderhit" => Some(Input::PonderHit),
        "quit" => Some(Input::Quit),
        x => {
            if x.starts_with("position") {
                parse_position_command(content)
            } else if x.starts_with("go") {
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
        static ref DEPTH: Regex = re(format!("depth {}", int_re().as_str()));
        static ref MOVETIME: Regex = re(format!("movetime {}", int_re().as_str()));
        static ref WHITETIME: Regex = re(format!("wtime {}", int_re().as_str()));
        static ref BLACKTIME: Regex = re(format!("btime {}", int_re().as_str()));
        static ref WHITEINC: Regex = re(format!("winc {}", int_re().as_str()));
        static ref BLACKINC: Regex = re(format!("binc {}", int_re().as_str()));
        static ref SEARCHMOVES: Regex =
            re(format!("searchmoves({}{})+", space_re().as_str(), move_re().as_str()));
    }
    let content_ref = content.as_str();
    let extract = |m: Match| int_re().find(m.as_str()).unwrap().as_str().parse::<usize>().unwrap();
    let mut dest = Vec::new();
    &INFINITE.find(content_ref).map(|_| dest.push(GoCommand::Infinite));
    &PONDER.find(content_ref).map(|_| dest.push(GoCommand::Ponder));
    &DEPTH.find(content_ref).map(|m| dest.push(GoCommand::Depth(extract(m))));
    &MOVETIME.find(content_ref).map(|m| dest.push(GoCommand::MoveTime(extract(m))));
    &WHITETIME.find(content_ref).map(|m| dest.push(GoCommand::WhiteTime(extract(m))));
    &BLACKTIME.find(content_ref).map(|m| dest.push(GoCommand::BlackTime(extract(m))));
    &WHITEINC.find(content_ref).map(|m| dest.push(GoCommand::WhiteInc(extract(m))));
    &BLACKINC.find(content_ref).map(|m| dest.push(GoCommand::BlackInc(extract(m))));
    &SEARCHMOVES.find(content_ref).map(|m| {
        let moves = move_re().find_iter(m.as_str()).map(|n| n.as_str().to_owned()).collect();
        dest.push(GoCommand::SearchMoves(moves));
    });
    dest
}

fn parse_position_command(content: String) -> Option<Input> {
    let split: Vec<String> = space_re().split(content.as_str()).map(|x| x.to_owned()).collect();
    if split.len() > 0 {
        let first = split.first().unwrap().to_owned();
        let rest = split.into_iter().skip(1).collect();
        Some(Input::Position(first, rest))
    } else {
        None
    }
}

fn int_re() -> &'static Regex {
    lazy_static! {
        static ref INT_RE: Regex = re(r"[0-9]+".to_owned());
    }
    &INT_RE
}
fn space_re() -> &'static Regex {
    lazy_static! {
        static ref WHITESPACE: Regex = re(r"\s+".to_owned());
    }
    &WHITESPACE
}

fn move_re() -> &'static Regex {
    lazy_static! {
        static ref MOVE: Regex = re(r"([a-h][1-8]){2}[qrnb]?".to_owned());
    }
    &MOVE
}

fn re(pattern: String) -> Regex {
    Regex::new(pattern.as_str()).unwrap()
}
