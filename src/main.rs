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
use regex::Regex;

mod base;
mod eval;
mod pgn;
mod pieces;
mod search;
mod board;

#[derive(Debug)]
enum EngineState {
    WaitingForGui,
    Initializing,
    WaitingForPosition,
    WaitingForGo,
    Searching,
    Pondering,
}

#[derive(Debug)]
enum EngineCmd {
    Uci,
    IsReady,
    UciNewGame,
    Stop,
    PonderHit,
    Quit,
    Position(String, Vec<String>),
    Go(Vec<GoCommand>),
}

#[derive(Debug)]
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
    let (cmd_input_tx, cmd_input_rx) = mpsc::channel::<EngineCmd>();
    // Spawn the user input thread, it simply listens for
    // standard input, parses the string to an engine command
    // and transmits the result (if valid) to the main
    // control thread.
    thread::spawn(move || {
        loop {
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
        }
    });

    loop {
        match cmd_input_rx.try_recv() {
            Err(_) => thread::sleep(Duration::from_millis(10)),
            Ok(input) => match input {
                EngineCmd::Quit => break,
                x => println!("{:?}", x),
            }
        }
    }
}

fn parse_engine_command(content: String) -> Option<EngineCmd> {
    let content_ref = content.as_str();
    match content_ref {
        "uci" => Some(EngineCmd::Uci),
        "isready" => Some(EngineCmd::IsReady),
        "ucinewgame" => Some(EngineCmd::UciNewGame),
        "stop" => Some(EngineCmd::Stop),
        "ponderhit" => Some(EngineCmd::PonderHit),
        "quit" => Some(EngineCmd::Quit),
        x => {
            if x.starts_with("position") {
                parse_position_command(content)
            } else if x.starts_with("go") {
                parse_go_command(content)
            } else {
                None
            }
        },
    }
}

fn parse_go_command(content: String) -> Option<EngineCmd> {
    unimplemented!()
}

fn parse_position_command(content: String) -> Option<EngineCmd> {
    let split: Vec<String> = space_re()
        .split(content.as_str()).map(|x| x.to_owned()).collect();
    if split.len() > 0 {
        let first = split.first().unwrap().to_owned();
        let rest = split.into_iter().skip(1).collect();
        Some(EngineCmd::Position(first, rest))
    } else {
        None
    }
}

fn space_re() -> &'static Regex {
    lazy_static! {
        static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
    }
    &WHITESPACE
}

fn move_re() -> &'static Regex {
    lazy_static! {
        static ref MOVE: Regex = Regex::new(r"([a-h][1-8]){2}[qrnb]?").unwrap();
    }
    &MOVE
}