#![allow(unused)]

use clap::Parser;
use lib::{euclid_distance::EuclidDistance, opt2::solve};

#[derive(Parser)]
struct Argument {
    problem_path: std::path::PathBuf,
}

fn main() {
    let args = Argument::parse();
    let distance = EuclidDistance::load_tsplib(&args.problem_path);
    let solution = solve(&distance);
}
