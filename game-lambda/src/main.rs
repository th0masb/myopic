extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use reqwest::blocking::{Client, Request, Response};
use std::io::{Read, BufReader, BufRead};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull {
        id: String,
        white: Player,
        black: Player,
        state: GameState,
    },

    #[serde(rename = "gameState")]
    State {
        #[serde(flatten)]
        state: GameState
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct GameState {
    moves: String,
    wtime: u64,
    btime: u64,
    winc: u64,
    binc: u64,
    wdraw: bool,
    bdraw: bool,
    status: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct Player {
    id: String,
    name: String,
    title: Option<String>,
    rating: usize,
    provisional: bool,
}

fn main() {
    let client = reqwest::blocking::Client::new();

    let response = client.get("https://lichess.org/api/bot/game/stream/mtjmNyVe")
        .bearer_auth("xxx")
        .send()
        .unwrap();

    let mut reader = BufReader::new(response);
    while let readResult = readline(&mut reader) {
        match readResult {
            ReadResult::End => break,
            ReadResult::Err => continue,
            ReadResult::Line(s) => {
                if !s.is_empty() {
                    match serde_json::from_str(s.as_str()) {
                        Err(error) => println!("Error during parse: {:?}", error),
                        Ok(event) => match event {
                            GameEvent::GameFull { .. } => println!("Parsed gameFull: {:?}", event),
                            GameEvent::State { .. } => println!("Parsed state: {:?}", event),
                        }
                    }
                }
            },
        }
    }
}

//fn main() {
//    let client = reqwest::blocking::Client::new();
//
//    let response = client.get("https://lichess.org/api/stream/event")
//        .bearer_auth("")
//        .send()
//        .unwrap();
//
//    let mut reader = BufReader::new(response);
//    while let readResult = readline(&mut reader) {
//        match readResult {
//            ReadResult::End => break,
//            ReadResult::Err => continue,
//            ReadResult::Line(s) => {
//                if !s.is_empty() {
//                    println!("{}\n", y)
//                }
//            },
//        }
//    }
//}

enum ReadResult {
    Line(String),
    Err,
    End,
}

fn readline<R: Read>(bufreader: &mut BufReader<R>) -> ReadResult {
    let mut dest = String::new();
    match bufreader.read_line(&mut dest) {
        Ok(0) => ReadResult::End,
        Ok(_) => ReadResult::Line(String::from(dest.trim())),
        _ => ReadResult::Err,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_state() {
        let json = r#"{
            "type": "gameState",
            "moves": "e2e4 c7c5",
            "wtime": 1000,
            "btime": 1000,
            "winc": 0,
            "binc": 0,
            "wdraw": false,
            "bdraw": false,
            "status": "started",
            "other": "value"
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!(format!("Parse error {:?}", error)),
            Ok(event) => match event {
                GameEvent::GameFull { .. }  => panic!(format!("Wrong event {:?}", event)),
                GameEvent::State { state } => assert_eq!(
                    GameState {
                        moves: String::from("e2e4 c7c5"),
                        wtime: 1000,
                        btime: 1000,
                        winc: 0,
                        binc: 0,
                        wdraw: false,
                        bdraw: false,
                        status: String::from("started")
                    }
                , state)
            }
        }
    }

    #[test]
    fn deserialize_game_full() {
        let json = r#"{
            "type": "gameFull",
            "id": "123",
            "other": "value",
            "white": {
                "id": "th0masb",
                "name": "th0masb",
                "title": null,
                "rating": 1500,
                "provisional": true,
                "other": "value"
            },
            "black": {
                "id": "myopic-bot",
                "name": "myopic-bot",
                "title": "BOT",
                "rating": 1500,
                "provisional": true
            },
            "state": {
                "moves": "e2e4 e7e5",
                "wtime": 1000,
                "btime": 1000,
                "winc": 0,
                "binc": 0,
                "wdraw": false,
                "bdraw": false,
                "status": "started"
            }
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!(format!("Parse error {:?}", error)),
            Ok(event) => match event {
                GameEvent::State { .. } => panic!(format!("Wrong type {:?}", event)),
                GameEvent::GameFull { id, white, black, state } => {
                    assert_eq!("123", id);
                    assert_eq!(Player {
                        id: String::from("th0masb"),
                        name: String::from("th0masb"),
                        title: None,
                        rating: 1500,
                        provisional: true
                    }, white);
                    assert_eq!(Player {
                        id: String::from("myopic-bot"),
                        name: String::from("myopic-bot"),
                        title: Some(String::from("BOT")),
                        rating: 1500,
                        provisional: true
                    }, black);
                    assert_eq!(GameState {
                        moves: String::from("e2e4 e7e5"),
                        wtime: 1000,
                        btime: 1000,
                        winc: 0,
                        binc: 0,
                        wdraw: false,
                        bdraw: false,
                        status: String::from("started")
                    }, state);
                }
            }
        }
    }
}

