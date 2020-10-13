extern crate reqwest;

use std::error::Error;
use reqwest::blocking::{Client, Request, Response};
use std::io::{Read, BufReader, BufRead};

fn main() {
    let client = reqwest::blocking::Client::new();

    let response = client.get("https://lichess.org/api/stream/event")
        .bearer_auth("h9aFqfXSa9mxQdze")
        .send()
        .unwrap();

    let mut reader = BufReader::new(response);
    while let readResult = readline(&mut reader) {
        match readResult {
            ReadResult::End => break,
            ReadResult::Err => continue,
            ReadResult::Line(s) => {
                let y = s.trim();
                if !y.is_empty() {
                    println!("{}\n", y)
                }
            },
        }
    }
}

enum ReadResult {
    Line(String),
    Err,
    End,
}

fn readline<R: Read>(bufreader: &mut BufReader<R>) -> ReadResult {
    let mut dest = String::new();
    match bufreader.read_line(&mut dest) {
        Ok(0) => ReadResult::End,
        Ok(_) => ReadResult::Line(dest),
        _ => ReadResult::Err,
    }
}
