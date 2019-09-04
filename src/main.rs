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

mod base;
mod eval;
mod pgn;
mod pieces;
mod search;
mod board;

#[derive(Debug)]
enum EngineState {
    Initializing,
    WaitingForPosition,
    WaitingForGo,
    Searching,
    Pondering
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

fn parse_engine_command(content: String) -> Option<EngineCmd> {
    unimplemented!()
}

fn main() {
    let (input_tx, input_rx) = mpsc::channel::<EngineCmd>();
    // Spawn the user input thread, it simply listens for
    // standard input, parses the string to an engine command
    // and transmits the result (if valid) to the main
    // control thread.
    thread::spawn(move || {
        loop {
            let mut dest = String::new();
            io::stdin().read_line(&mut dest);
            let cmd = parse_engine_command(dest);
            if cmd.is_some() {
                match input_tx.send(cmd.unwrap()) {
                    Ok(_) => (),
                    Err(_) => continue,
                }
            }
        }
    });

    let timer = Instant::now();
    loop {
        if timer.elapsed().as_secs() >= 60 {
            break;
        }
        match input_rx.try_recv() {
            Ok(input) => println!("Recieved: {:?}", input),
            Err(_) => thread::sleep(Duration::from_millis(10)),
        }
    }

    println!("quitting");
}
