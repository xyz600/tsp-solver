use std::array;
use std::time::Instant;

use lib::array_solution::ArraySolution;
use lib::solution::Solution;
use lib::two_level_tree_solution::TwoLeveltreeSolution;
use rand::Rng;

fn main() {
    let n_city = 100_000;
    let mut array_solution = ArraySolution::new(n_city);
    let mut two_level_tree_solution = TwoLeveltreeSolution::<1000>::new(&array_solution);

    let mut rng = rand::thread_rng();

    let mut input_list = vec![];
    for _iter in 0..20000 {
        let from = rng.gen_range(0..n_city as u32);
        let to = rng.gen_range(0..(n_city as u32) - 1);
        let to = if to == from { to + 1 } else { to };
        input_list.push((from, to));
    }

    let start = Instant::now();
    for (from, to) in input_list.iter() {
        array_solution.swap(*from, *to);
    }
    let elapsed = (Instant::now() - start).as_millis();
    eprintln!("array solution: {}[ms]", elapsed);

    let start = Instant::now();
    for (from, to) in input_list.iter() {
        two_level_tree_solution.swap(*from, *to);
    }
    let elapsed = (Instant::now() - start).as_millis();
    eprintln!("two-level-tree solution: {}[ms]", elapsed);

    // check
    for id in 0..n_city as u32 {
        if array_solution.next(id) != two_level_tree_solution.next(id) {
            eprintln!("ERROR next: id = {}", id);
        }
        if array_solution.prev(id) != two_level_tree_solution.prev(id) {
            eprintln!("ERROR prev: id = {}", id);
        }
        assert_eq!(array_solution.prev(id), two_level_tree_solution.prev(id));
    }
}
