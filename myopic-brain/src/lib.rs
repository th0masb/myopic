extern crate myopic_core;
extern crate myopic_board;
extern crate itertools;

pub mod eval;
pub mod search;
mod evalboardimpl;
mod quiescent;
mod see;
mod tables;
mod values;

#[cfg(test)]
mod mate_benchmark;