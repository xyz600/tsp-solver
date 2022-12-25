use crate::{
    array_solution::ArraySolution, bitset::BitSet, distance::DistanceFunction, intset::IntSet,
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

#[derive(Debug)]
enum NeighborPattern {
    None,
    Pat1((u32, u32)),
    Pat2((u32, u32), (u32, u32)),
    Pat3((u32, u32), (u32, u32), (u32, u32)),
}

// https://en.wikipedia.org/wiki/3-opt
pub fn solve(distance: &(impl DistanceFunction + std::marker::Sync)) -> ArraySolution {
    let solution = ArraySolution::new(distance.dimension() as usize);
    let n = solution.len();

    let mut tlt = TwoLeveltreeSolution::<1000>::new(&solution);

    let neighbor_table = NeighborTable::new(distance, 50);
    let mut rng = rand::thread_rng();

    let mut dlb = IntSet::new(n);
    dlb.set_all();

    let mut eval = evaluate(distance, &tlt);
    let mut selected = BitSet::new(n);

    for iter in 0.. {
        let a = dlb.random_select(&mut rng);

        selected.clear_all();

        let mut best_gain = 0;
        let mut best_pat = NeighborPattern::None;

        let a_next = tlt.next(a);
        let a_prev = tlt.prev(a);

        for (a, b) in [(a_prev, a), (a, a_next)] {
            selected.set(a);
            selected.set(b);

            for c in neighbor_table.neighbor_list(a) {
                let c_next = tlt.next(*c);
                let c_prev = tlt.prev(*c);

                for (c, d) in [(c_prev, *c), (*c, c_next)] {
                    if selected.test(c) || selected.test(d) {
                        continue;
                    }
                    selected.set(c);
                    selected.set(d);

                    for e in neighbor_table.neighbor_list(c) {
                        let e_next = tlt.next(*e);
                        let e_prev = tlt.prev(*e);

                        for (e, f) in [(e_prev, *e), (*e, e_next)] {
                            if selected.test(e) || selected.test(f) {
                                continue;
                            }
                            selected.set(e);
                            selected.set(f);

                            let dist = |i1, i2| distance.distance(i1, i2);

                            // case 1
                            // [(a, b), (c, d)] -> [(a, c), (b, d)]
                            let gain1 = dist(a, b) + dist(c, d) - dist(a, c) - dist(b, d);
                            if gain1 > best_gain {
                                best_gain = gain1;
                                best_pat = NeighborPattern::Pat1((b, c))
                            }

                            // case 2
                            // [(c, d), (e, f)] -> [(c, e), (d, f)]
                            let gain2 = dist(c, d) + dist(e, f) - dist(c, e) - dist(d, f);
                            if gain2 > best_gain {
                                best_gain = gain2;
                                best_pat = NeighborPattern::Pat1((d, e));
                            }

                            // case 3
                            // [(a, b), (e, f)] -> [(a, e), (b, f)]
                            let gain3 = dist(a, b) + dist(e, f) - dist(a, e) - dist(b, f);
                            if gain3 > best_gain {
                                best_gain = gain3;
                                best_pat = NeighborPattern::Pat1((f, a));
                            }

                            // case 4
                            // [(a, b), (c, d), (e, f)] -> [(a, c), (b, e), (d, f)]
                            // [(a, b), (c, d), (e, f)] -> [(a, c), (b, d), (e, f)] -> [(a, c), (b, e), (d, f)]
                            let gain4 = dist(a, b) + dist(c, d) + dist(e, f)
                                - dist(a, c)
                                - dist(b, e)
                                - dist(d, f);
                            if gain4 > best_gain {
                                best_gain = gain4;
                                best_pat = NeighborPattern::Pat2((b, c), (d, e));
                            }

                            // case 5
                            // [(a, b), (c, d), (e, f)] -> [(a, e), (d, b), (c, f)]
                            // [(a, b), (c, d), (e, f)] -> [(a, c), (b, d), (e, f)] -> [(a, e), (d, b), (c, f)]
                            let gain5 = dist(a, b) + dist(c, d) + dist(e, f)
                                - dist(a, e)
                                - dist(d, b)
                                - dist(c, f);
                            if gain5 > best_gain {
                                best_gain = gain5;
                                best_pat = NeighborPattern::Pat2((b, c), (c, e));
                            }

                            // case 6
                            // [(a, b), (c, d), (e, f)] -> [(a, d), (e, c), (b, f)]
                            // [(a, b), (c, d), (e, f)] -> [(a, e), (d, c), (b, f)] -> [(a, d), (e, c), (b, f)]
                            let gain6 = dist(a, b) + dist(c, d) + dist(e, f)
                                - dist(a, d)
                                - dist(e, c)
                                - dist(b, f);
                            if gain6 > best_gain {
                                best_gain = gain6;
                                best_pat = NeighborPattern::Pat2((b, e), (e, d));
                            }

                            // case 7
                            // [(a, b), (c, d), (e, f)] -> [(a, d), (e, b), (c, f)]
                            // [(a, b), (c, d), (e, f)] -> [(a, e), (d, c), (b, f)] -> [(a, d), (e, c), (b, f)] -> [(a, d), (e, b), (c, f)]
                            let gain7 = dist(a, b) + dist(c, d) + dist(e, f)
                                - dist(a, d)
                                - dist(e, b)
                                - dist(c, f);
                            if gain7 > best_gain {
                                best_gain = gain7;
                                best_pat = NeighborPattern::Pat3((b, e), (e, d), (c, b));
                            }

                            selected.clear(e);
                            selected.clear(f);
                        }
                    }

                    selected.clear(c);
                    selected.clear(d);
                }
            }

            selected.clear(a);
            selected.clear(b);
        }

        if best_gain > 0 {
            // swap
            match best_pat {
                NeighborPattern::None => unreachable!(),
                NeighborPattern::Pat1((i1, i2)) => {
                    tlt.swap(i1, i2);
                    for i in [i1, i2] {
                        dlb.push(i);
                    }
                    eval -= best_gain;
                }
                NeighborPattern::Pat2((i1, i2), (i3, i4)) => {
                    tlt.swap(i1, i2);
                    tlt.swap(i3, i4);
                    for i in [i1, i2, i3, i4] {
                        dlb.push(i);
                    }
                    eval -= best_gain;
                }
                NeighborPattern::Pat3((i1, i2), (i3, i4), (i5, i6)) => {
                    tlt.swap(i1, i2);
                    tlt.swap(i3, i4);
                    tlt.swap(i5, i6);
                    for i in [i1, i2, i3, i4, i5, i6] {
                        dlb.push(i);
                    }
                    eval -= best_gain;
                }
            }
        } else {
            dlb.remove(a);
        }

        if iter % (n / 100) == 0 || dlb.is_empty() {
            eprintln!("iter = {}, eval = {}", iter, eval);
            eprintln!("dlb size = {}", dlb.len());
        }
        if dlb.is_empty() {
            break;
        }
    }
    solution
}
