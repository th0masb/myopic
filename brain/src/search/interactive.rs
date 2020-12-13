use crate::search::{search, NegamaxContext, NegamaxTerminator};
use crate::{EvalBoard, SearchOutcome};
use myopic_core::Side;
use std::cmp::{max, min};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

const INFINITE_DURATION: Duration = Duration::from_secs(1_000_000);
const INFINITE_DEPTH: usize = 1_000;
const DEFAULT_SEARCH_DURATION: Duration = Duration::from_secs(30);
const DEFAULT_SEARCH_DEPTH: usize = 10;
const MAX_COMPUTED_MOVE_SEARCH_DURATION: Duration = Duration::from_secs(45);

pub type InteractiveSearchCommandTx<B> = Sender<InteractiveSearchCommand<B>>;
pub type InteractiveSearchResultRx = Receiver<Result<SearchOutcome, String>>;
type CmdRx<B> = Receiver<InteractiveSearchCommand<B>>;
type ResultTx = Sender<Result<SearchOutcome, String>>;

#[derive(Debug, Clone, PartialEq)]
pub enum InteractiveSearchCommand<B: EvalBoard> {
    Go,
    GoOnce,
    Stop,
    Close,
    Root(B),
    Infinite,
    Depth(usize),
    Time(usize),
    GameTime { w_base: usize, w_inc: usize, b_base: usize, b_inc: usize },
}

/// Create an interactive search running on a separate thread, communication happens
/// via an input channel which accepts a variety of commands and an output channel
/// which transmits the search results.
pub fn interactive_search<B: EvalBoard + 'static>(
) -> (InteractiveSearchCommandTx<B>, InteractiveSearchResultRx) {
    let (input_tx, input_rx) = mpsc::channel::<InteractiveSearchCommand<B>>();
    let (output_tx, output_rx) = mpsc::channel::<Result<SearchOutcome, String>>();
    std::thread::spawn(move || {
        let mut search = InteractiveSearch::new(input_rx, output_tx);
        loop {
            match &search.input_rx.recv() {
                Err(_) => continue,
                Ok(input) => match input.to_owned() {
                    InteractiveSearchCommand::Close => break,
                    InteractiveSearchCommand::Stop => (),
                    InteractiveSearchCommand::Go => search.execute_then_send(),
                    InteractiveSearchCommand::Root(root) => search.root = Some(root),
                    InteractiveSearchCommand::Depth(max_depth) => search.max_depth = max_depth,
                    InteractiveSearchCommand::Time(max_time) => search.set_max_time(max_time),
                    InteractiveSearchCommand::GameTime { w_base, w_inc, b_base, b_inc } => {
                        search.set_game_time(w_base, w_inc, b_base, b_inc)
                    }
                    InteractiveSearchCommand::Infinite => {
                        search.max_time = INFINITE_DURATION;
                        search.max_depth = INFINITE_DEPTH;
                    }
                    InteractiveSearchCommand::GoOnce => {
                        search.execute_then_send();
                        break;
                    }
                },
            }
        }
    });
    (input_tx, output_rx)
}

struct InteractiveSearch<B: EvalBoard> {
    input_rx: Rc<CmdRx<B>>,
    output_tx: ResultTx,
    root: Option<B>,
    max_depth: usize,
    max_time: Duration,
}

impl<B: EvalBoard + 'static> InteractiveSearch<B> {
    pub fn new(input_rx: CmdRx<B>, output_tx: ResultTx) -> InteractiveSearch<B> {
        InteractiveSearch {
            input_rx: Rc::new(input_rx),
            root: None,
            output_tx,
            max_depth: DEFAULT_SEARCH_DEPTH,
            max_time: DEFAULT_SEARCH_DURATION,
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
            self.set_max_time(min(time, MAX_COMPUTED_MOVE_SEARCH_DURATION.as_millis() as usize));
        }
    }

    pub fn execute_then_send(&self) -> () {
        if self.root.is_some() {
            match self.output_tx.send(self.execute()) {
                _ => (),
            }
        }
    }

    pub fn execute(&self) -> Result<SearchOutcome, String> {
        let tracker = InteractiveSearchTerminator {
            max_depth: self.max_depth,
            max_time: self.max_time,
            stop_signal: self.input_rx.clone(),
        };
        search(self.root.clone().unwrap(), tracker)
    }
}

struct InteractiveSearchTerminator<B: EvalBoard> {
    max_time: Duration,
    max_depth: usize,
    stop_signal: Rc<CmdRx<B>>,
}

impl<B: EvalBoard> NegamaxTerminator for InteractiveSearchTerminator<B> {
    fn should_terminate(&self, ctx: &NegamaxContext) -> bool {
        ctx.start_time.elapsed() > self.max_time
            || ctx.depth_remaining >= self.max_depth
            || match self.stop_signal.try_recv() {
                Ok(InteractiveSearchCommand::Stop) => true,
                _ => false,
            }
    }
}
