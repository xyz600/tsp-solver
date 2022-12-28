#![allow(unused)]

use clap::Parser;
use lib::{
    array_solution::ArraySolution, distance::DistanceFunction, euclid_distance::EuclidDistance,
    lkh, opt2, opt3,
};

#[derive(Parser)]
struct Argument {
    problem_path: std::path::PathBuf,
}

fn main() {
    let args = Argument::parse();
    let distance = EuclidDistance::load_tsplib(&args.problem_path);
    let solution = ArraySolution::new(distance.dimension() as usize);
    let solution = opt2::solve(&distance, solution);
    let solution = opt3::solve(&distance, solution);
    let solution = lkh::solve(&distance, solution);
}
