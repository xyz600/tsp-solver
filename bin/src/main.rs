#![allow(unused)]

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use lib::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    divide_and_conqure_solver::{self, DivideAndConqureConfig},
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

    let mut solution = lkh::solve(
        &distance,
        solution,
        LKHConfig {
            use_neighbor_cache: true,
            cache_filepath: PathBuf::from_str(cache_filepath.as_str()).unwrap(),
            debug: false,
            time_ms: 60_000,
            start_kick_step: 30,
            kick_step_diff: 10,
            end_kick_step: distance.dimension() as usize / 10,
            fail_count_threashold: 50,
            max_depth: 6,
        },
    );
    eprintln!("finish initial lkh.");
    eprintln!("eval = {}", evaluate(&distance, &solution));

    // 分割して並列化

    let mut best_eval = evaluate(&distance, &solution);
    let mut start_kick_step = 30;
    let mut time_ms = 30_000;

    for iter in 1.. {
        solution = divide_and_conqure_solver::solve(
            &distance,
            &solution,
            DivideAndConqureConfig {
                no_split: 12,
                debug: false,
                time_ms,
                start_kick_step,
                kick_step_diff: 10,
                end_kick_step: distance.dimension() as usize / 10,
                fail_count_threashold: 50,
                max_depth: 7,
            },
        );
        let eval = evaluate(&distance, &solution);
        eprintln!("finish splited lkh {} times.", iter);
        eprintln!("eval = {}", eval);
        if best_eval == eval {
            start_kick_step += 10;
            time_ms += 30_000;
        }
        best_eval = eval;

        if start_kick_step == 100 {
            break;
        }
    }

    let mut solution = lkh::solve(
        &distance,
        solution,
        LKHConfig {
            use_neighbor_cache: true,
            cache_filepath: PathBuf::from_str(cache_filepath.as_str()).unwrap(),
            debug: false,
            time_ms: 24 * 60 * 60 * 1_000,
            start_kick_step: 30,
            kick_step_diff: 10,
            end_kick_step: distance.dimension() as usize / 10,
            fail_count_threashold: 50,
            max_depth: 7,
        },
    );
    eprintln!("finish initial lkh.");
    eprintln!("eval = {}", evaluate(&distance, &solution));
}
