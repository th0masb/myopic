mod tuning;
mod cartesian;

use crate::tuning::Tuner;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use myopic_brain::{
    Board, ChessBoard, EvalBoard, EvalConfig, Reflectable, SearchOutcome, SearchParameters, Side,
    Termination,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_derive::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

const MOVE_LIMIT: usize = 500;

#[derive(Debug, StructOpt, Serialize)]
#[structopt(name = "arena")]
struct Opt {
    /// Path to a json file which contains the tuning
    /// configuration parameters.
    #[structopt(long = "tuning-params", parse(from_os_str))]
    tuning_params: PathBuf,
    #[structopt(long = "start-positions", parse(from_os_str))]
    start_positions: PathBuf,
    /// The number of threads we are going to use to
    /// process all the games.
    #[structopt(long, default_value = "1")]
    threads: usize,
    #[structopt(long = "table-size")]
    table_size: usize,
    #[structopt(long, default_value = "2")]
    depth: usize,
    #[structopt(long = "move-limit", default_value = "500")]
    move_limit: usize,
    #[structopt(long, default_value = "5")]
    top: usize,
}

#[derive(Clone, Serialize)]
struct Competitor(Vec<EvalConfig>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();
    // Config should have at most one of each tuner type.
    let conf = std::fs::read_to_string(&opt.tuning_params)?;
    let positions = BufReader::new(std::fs::File::open(opt.start_positions)?)
        .lines()
        .filter_map(|l| l.ok())
        .collect_vec();
    let tuning_options = serde_json::from_str::<Vec<Tuner>>(conf.as_str())?;

    let mut competitors = cartesian::product(
        &tuning_options
            .into_iter()
            .map(|t| t.combinations())
            .collect_vec(),
    )
    .into_iter()
    .map(|es| Competitor(es))
    .collect_vec();

    competitors.shuffle(&mut thread_rng());

    while competitors.len() > opt.top {
        let n = competitors.len();
        let mut next_competitors = vec![];
        for i in (0..n - 1).step_by(2) {
            let (a, b) = (competitors[i].clone(), competitors[i + 1].clone());
            let PlayOutcome {
                a, b, awins, bwins, ..
            } = play_all(
                a,
                b,
                &positions,
                &PlayParams {
                    depth: opt.depth,
                    table_size: opt.table_size,
                    move_limit: MOVE_LIMIT,
                },
            )?;
            next_competitors.push(if awins >= bwins { a } else { b });
        }
        competitors = next_competitors;
    }

    println!(
        "{:?}",
        competitors
            .iter()
            .filter_map(|c| serde_json::to_string(c).ok())
            .collect_vec()
    );
    Ok(())
}

struct PlayOutcome {
    a: Competitor,
    b: Competitor,
    awins: usize,
    draws: usize,
    bwins: usize,
}

fn play_all(
    a: Competitor,
    b: Competitor,
    start_positions: &Vec<String>,
    params: &PlayParams,
) -> Result<PlayOutcome> {
    let (mut awins, mut draws, mut bwins) = (0, 0, 0);
    for pos in start_positions {
        let mut start_position = myopic_brain::start();
        start_position.play_uci(pos.as_str())?;

        // Each competitor plays from start position on each side
        let awhite_winner = play_one(&a, &b, start_position.clone(), params)?;
        let bwhite_winner = play_one(&b, &a, start_position.clone(), params)?;
        match (awhite_winner, bwhite_winner) {
            (None, None) => draws += 1,
            (Some(Side::White), None) => awins += 1,
            (Some(Side::Black), None) => bwins += 1,
            (None, Some(Side::White)) => bwins += 1,
            (None, Some(Side::Black)) => awins += 1,
            (Some(Side::White), Some(Side::White)) => draws += 1,
            (Some(Side::Black), Some(Side::Black)) => draws += 1,
            (Some(Side::White), Some(Side::Black)) => awins += 1,
            (Some(Side::Black), Some(Side::White)) => bwins += 1,
        }
    }

    Ok(PlayOutcome {
        a,
        b,
        awins,
        draws,
        bwins,
    })
}

struct PlayParams {
    depth: usize,
    table_size: usize,
    move_limit: usize,
}

fn play_one(
    white: &Competitor,
    black: &Competitor,
    mut start: Board,
    params: &PlayParams,
) -> Result<Option<Side>> {
    let mut i = 0;
    while start.termination_status().is_none() && i < params.move_limit {
        i += 1;
        match start.active() {
            Side::White => {
                make_move(white, &mut start, params)?;
            }
            Side::Black => {
                make_move(black, &mut start, params)?;
            }
        }
    }
    Ok(match (start.termination_status(), start.active()) {
        (None, _) => None,
        (Some(Termination::Loss), s) => Some(s.reflect()),
        (Some(Termination::Draw), _) => None,
    })
}

fn make_move(player: &Competitor, start: &mut Board, params: &PlayParams) -> Result<()> {
    let SearchOutcome { best_move, .. } = myopic_brain::search(
        EvalBoard::builder(start.clone())
            .configure_all(player.0.clone())
            .build(),
        SearchParameters {
            terminator: params.depth,
            table_size: params.table_size,
        },
    )?;
    start.make(best_move)?;
    Ok(())
}
