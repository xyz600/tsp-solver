#![allow(unused)]

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use lib::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    euclid_distance::EuclidDistance,
    evaluate::evaluate,
    lkh::{self, LKHConfig},
    opt2::{self, Opt2Config},
    opt3::{self, Opt3Config},
};

#[derive(Parser)]
struct Argument {
    problem_path: std::path::PathBuf,
}

fn get_default_cache_filepath(distance: &impl DistanceFunction) -> String {
    format!("{}.cache", distance.name())
}

fn main() {
    let args = Argument::parse();
    let distance = EuclidDistance::load_tsplib(&args.problem_path);
    let solution = ArraySolution::new(distance.dimension() as usize);

    let cache_filepath = get_default_cache_filepath(&distance);

    let solution = opt2::solve(
        &distance,
        solution,
        Opt2Config {
            use_neighbor_cache: true,
            cache_filepath: PathBuf::from_str(cache_filepath.as_str()).unwrap(),
            debug: false,
        },
    );
    eprintln!("finish 2-opt.");
    eprintln!("eval = {}", evaluate(&distance, &solution));

    let solution = opt3::solve(
        &distance,
        solution,
        Opt3Config {
            use_neighbor_cache: true,
            cache_filepath: PathBuf::from_str(cache_filepath.as_str()).unwrap(),
            debug: false,
        },
    );
    eprintln!("finish 3-opt.");
    eprintln!("eval = {}", evaluate(&distance, &solution));

    let solution = lkh::solve(
        &distance,
        solution,
        LKHConfig {
            use_neighbor_cache: true,
            cache_filepath: PathBuf::from_str(cache_filepath.as_str()).unwrap(),
            debug: false,
            time_ms: 120_000,
            start_kick_step: 30,
            kick_step_diff: 10,
            end_kick_step: distance.dimension() as usize / 10,
            fail_count_threashold: 50,
            max_depth: 6,
        },
    );
    eprintln!("finish lkh.");
    eprintln!("eval = {}", evaluate(&distance, &solution));
}
