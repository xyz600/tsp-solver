use crate::{
    array_solution::ArraySolution, distance::DistanceFunction, intset::IntSet,
    neighbor_table::NeighborTable, solution::Solution,
    two_level_tree_solution::TwoLeveltreeSolution,
};

fn evaluate(distance: &impl DistanceFunction, solution: &impl Solution) -> i64 {
    let mut sum = 0;
    let mut id = 0;
    for _iter in 0..distance.dimension() {
        let next = solution.next(id);
        sum += distance.distance(id, next);
        id = next;
    }
    sum
}

pub fn solve(distance: &(impl DistanceFunction + std::marker::Sync)) -> ArraySolution {
    let solution = ArraySolution::new(distance.dimension() as usize);
    let n = solution.len();

    let mut tlt = TwoLeveltreeSolution::<1000>::new(&solution);

    let neighbor_table = NeighborTable::new(distance, 50);
    let mut rng = rand::thread_rng();

    let mut dlb = IntSet::new(n);
    dlb.set_all();

    let mut eval = evaluate(distance, &tlt);

    for iter in 0.. {
        let a = dlb.random_select(&mut rng);
        let b = tlt.next(a);

        let mut best_gain = 0;
        let mut best_c = 0;

        for c in neighbor_table.neighbor_list(a).iter() {
            let c = *c;
            if c == a || c == b {
                continue;
            }
            let d = tlt.next(c);
            if a == d || b == d {
                continue;
            }

            let gain = distance.distance(a, b) + distance.distance(c, d)
                - distance.distance(a, c)
                - distance.distance(b, d);
            if gain > best_gain {
                best_gain = gain;
                best_c = c;
            }
        }
        if best_gain > 0 {
            dlb.push(a);
            dlb.push(b);
            dlb.push(best_c);
            let best_d = tlt.next(best_c);
            dlb.push(best_d);

            tlt.swap(b, best_c);
            eval -= best_gain;
        } else {
            dlb.remove(a);
        }

        if iter % n == 0 || dlb.is_empty() {
            eprintln!("iter = {}, eval = {}", iter, eval);
            eprintln!("dlb size = {}", dlb.len());
        }
        if dlb.is_empty() {
            break;
        }
    }
    solution
}
