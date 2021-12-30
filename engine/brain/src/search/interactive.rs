use std::cmp::{max, min};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use myopic_board::anyhow::Result;
use myopic_board::Side;

use crate::{EvalChessBoard, SearchOutcome};
use crate::search::{search as blocking_search, SearchContext, SearchParameters, SearchTerminator};

const INFINITE_DURATION: Duration = Duration::from_secs(1_000_000);
const INFINITE_DEPTH: usize = 1_000;
const DEFAULT_SEARCH_DURATION: Duration = Duration::from_secs(30);
const DEFAULT_SEARCH_DEPTH: usize = 10;
const DEFAULT_TABLE_SIZE: usize = 100_000;
const MAX_COMPUTED_MOVE_SEARCH_DURATION: Duration = Duration::from_secs(45);

pub type SearchCommandTx<B> = Sender<SearchCommand<B>>;
pub type SearchResultRx = Receiver<Result<SearchOutcome>>;
type CmdRx<B> = Receiver<SearchCommand<B>>;
type ResultTx = Sender<Result<SearchOutcome>>;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchCommand<B: EvalChessBoard> {
    Go,
    GoOnce,
    Stop,
    Close,
    Root(B),
    Infinite,
    Depth(usize),
    Time(usize),
    GameTime {
        w_base: usize,
        w_inc: usize,
        b_base: usize,
        b_inc: usize,
    },
}

/// Create an interactive search running on a separate thread, communication happens
/// via an input channel which accepts a variety of commands and an output channel
/// which transmits the search results.
/// TODO How to best handle need for board to be Send + Sync when it uses Rc?
pub fn search<B: EvalChessBoard + Send + Sync + 'static>() -> (SearchCommandTx<B>, SearchResultRx) {
    let (input_tx, input_rx) = mpsc::channel::<SearchCommand<B>>();
    let (output_tx, output_rx) = mpsc::channel::<Result<SearchOutcome>>();
    std::thread::spawn(move || {
        let mut search = InteractiveSearch::new(input_rx, output_tx);
        loop {
            match &search.input_rx.recv() {
                Err(_) => continue,
                Ok(input) => match input.to_owned() {
                    SearchCommand::Close => break,
                    SearchCommand::Stop => (),
                    SearchCommand::Go => search.execute_then_send(),
                    SearchCommand::Root(root) => search.root = Some(root),
                    SearchCommand::Depth(max_depth) => search.max_depth = max_depth,
                    SearchCommand::Time(max_time) => search.set_max_time(max_time),
                    SearchCommand::GameTime {
                        w_base,
                        w_inc,
                        b_base,
                        b_inc,
                    } => search.set_game_time(w_base, w_inc, b_base, b_inc),
                    SearchCommand::Infinite => {
                        search.max_time = INFINITE_DURATION;
                        search.max_depth = INFINITE_DEPTH;
                    }
                    SearchCommand::GoOnce => {
                        search.execute_then_send();
                        break;
                    }
                },
            }
        }
    });
    (input_tx, output_rx)
}

struct InteractiveSearch<B: EvalChessBoard> {
    input_rx: Rc<CmdRx<B>>,
    output_tx: ResultTx,
    root: Option<B>,
    max_depth: usize,
    max_time: Duration,
    transposition_table_size: usize,
}

impl<B: EvalChessBoard + 'static> InteractiveSearch<B> {
    pub fn new(input_rx: CmdRx<B>, output_tx: ResultTx) -> InteractiveSearch<B> {
        InteractiveSearch {
            input_rx: Rc::new(input_rx),
            root: None,
            output_tx,
            max_depth: DEFAULT_SEARCH_DEPTH,
            max_time: DEFAULT_SEARCH_DURATION,
            transposition_table_size: DEFAULT_TABLE_SIZE,
        }
    }

    pub fn set_max_time(&mut self, time: usize) {
        self.max_time = Duration::from_millis(time as u64);
    }

    // TODO This lets time run out with an increment...
    pub fn set_game_time(&mut self, w_base: usize, w_inc: usize, b_base: usize, b_inc: usize) {
        if self.root.is_some() {
            let active = self.root.as_ref().unwrap().active();
            let mut time = max(
                500,
                match active {
                    Side::White => w_inc,
                    _ => b_inc,
                },
            );
            time += match active {
                Side::White => w_base / 10,
                Side::Black => b_base / 10,
            };
            self.set_max_time(min(
                time,
                MAX_COMPUTED_MOVE_SEARCH_DURATION.as_millis() as usize,
            ));
        }
    }

    pub fn execute_then_send(&self) -> () {
        if self.root.is_some() {
            match self.output_tx.send(self.execute()) {
                _ => (),
            }
        }
    }

    pub fn execute(&self) -> Result<SearchOutcome> {
        let tracker = InteractiveSearchTerminator {
            max_depth: self.max_depth,
            max_time: self.max_time,
            stop_signal: self.input_rx.clone(),
        };
        blocking_search(
            self.root.clone().unwrap(),
            SearchParameters {
                terminator: tracker,
                table_size: self.transposition_table_size,
            },
        )
    }
}

struct InteractiveSearchTerminator<B: EvalChessBoard> {
    max_time: Duration,
    max_depth: usize,
    stop_signal: Rc<CmdRx<B>>,
}

impl<B: EvalChessBoard> SearchTerminator for InteractiveSearchTerminator<B> {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        ctx.start_time.elapsed() > self.max_time
            || ctx.depth_remaining >= self.max_depth
            || match self.stop_signal.try_recv() {
                Ok(SearchCommand::Stop) => true,
                _ => false,
            }
    }
}
