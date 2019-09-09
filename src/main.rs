//#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::board::{BoardImpl, Move};
use regex::{Match, Regex};
use std::sync::mpsc::Sender;

mod base;
mod board;
mod eval;
mod pgn;
mod pieces;
mod search;

#[derive(Debug, Eq, PartialEq)]
enum EngineState {
    WaitingForGui,
    Initializing,
    WaitingForPosition,
    WaitingForGo,
    Searching,
    Pondering,
}

#[derive(Debug, Eq, PartialEq)]
enum EngineCommand {
    Uci,
    IsReady,
    UciNewGame,
    Stop,
    PonderHit,
    Quit,
    Position(String, Vec<String>),
    Go(Vec<GoCommand>),
}

#[derive(Debug, Eq, PartialEq)]
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

fn main() {
    // Engine input command channel
    let (cmd_input_tx, cmd_input_rx) = mpsc::channel::<EngineCommand>();
    // Spawn the thread responsible for registering user input.
    spawn_input_thread(cmd_input_tx);
    // Begin the main control loop
    loop {
        match cmd_input_rx.recv() {
            Err(_) => break,
            Ok(input) => match input {
                EngineCommand::Quit => break,
                x => println!("{:?}", x),
            },
        }
    }
}

/// Spawn a user input thread, it simply listens for
/// standard input, parses the string to an engine command
/// and transmits the result (if valid) along the given
/// sender instance.
fn spawn_input_thread(input_tx: Sender<EngineCommand>) {
    thread::spawn(move || loop {
        let mut dest = String::new();
        match io::stdin().read_line(&mut dest) {
            Ok(_) => (),
            Err(_) => continue,
        }
        let cmd = parse_engine_command(dest.trim().to_lowercase().to_owned());
        if cmd.is_some() {
            match input_tx.send(cmd.unwrap()) {
                _ => (),
            }
        }
    });
}

fn parse_engine_command(content: String) -> Option<EngineCommand> {
    match content.as_str() {
        "uci" => Some(EngineCommand::Uci),
        "isready" => Some(EngineCommand::IsReady),
        "ucinewgame" => Some(EngineCommand::UciNewGame),
        "stop" => Some(EngineCommand::Stop),
        "ponderhit" => Some(EngineCommand::PonderHit),
        "quit" => Some(EngineCommand::Quit),
        x => {
            if x.starts_with("position") {
                parse_position_command(content)
            } else if x.starts_with("go") {
                Some(EngineCommand::Go(parse_go_command(content)))
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

fn parse_position_command(content: String) -> Option<EngineCommand> {
    let split: Vec<String> = space_re().split(content.as_str()).map(|x| x.to_owned()).collect();
    if split.len() > 0 {
        let first = split.first().unwrap().to_owned();
        let rest = split.into_iter().skip(1).collect();
        Some(EngineCommand::Position(first, rest))
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
