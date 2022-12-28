use std::{path::PathBuf, str::FromStr, time::Instant};

use crate::{
    array_solution::ArraySolution, bitset::BitSet, distance::DistanceFunction, intset::IntSet,
    neighbor_table::NeighborTable, segment_tree::SegmentTree, solution::Solution,
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

fn solve_inner<'a, T: Solution>(
    depth: usize,
    max_depth: usize,
    distance: &impl DistanceFunction,
    neighbor_table: &NeighborTable,
    current_flip: &mut SegmentTree<'a, T>,
    best_flip: &mut SegmentTree<'a, T>,
    edge_stack: &mut Vec<(u32, u32)>,
    gain: i64,
    best_gain: &mut i64,
    selected: &mut BitSet,
) {
    if depth == max_depth {
        // 評価して最も良いゲインのものを保存
        if *best_gain < gain {
            *best_gain = gain;
            best_flip.copy_from(&current_flip);
        }
        return;
    }

    fn check<'a, T: Solution>(
        depth: usize,
        max_depth: usize,
        distance: &impl DistanceFunction,
        neighbor_table: &NeighborTable,
        current_flip: &mut SegmentTree<'a, T>,
        best_flip: &mut SegmentTree<'a, T>,
        edge_stack: &mut Vec<(u32, u32)>,
        gain: i64,
        best_gain: &mut i64,
        selected: &mut BitSet,
        f1: u32,
        t1: u32,
        f2: u32,
        t2: u32,
    ) {
        if selected.test(f2) || selected.test(t2) {
            return;
        }
        selected.set(f2);
        selected.set(t2);
        current_flip.swap(t1, f2);

        let partial_gain = distance.distance(f1, t1) + distance.distance(f2, t2)
            - distance.distance(f1, f2)
            - distance.distance(t1, t2);

        // 新しくできた (f1, f2), (t1, t2) というエッジが次の交換対象
        // 2パターンあるのでどちらも探索
        for edge in [(f1, f2), (t1, t2)] {
            edge_stack.push(edge);
            solve_inner(
                depth + 1,
                max_depth,
                distance,
                neighbor_table,
                current_flip,
                best_flip,
                edge_stack,
                gain + partial_gain,
                best_gain,
                selected,
            );
            edge_stack.pop();
        }

        current_flip.undo();
        selected.clear(f2);
        selected.clear(t2);
    }

    // edge stack のトップと入れ替える
    // from, to のどちらかに近い頂点を候補に入れたい
    let &(f1, t1) = edge_stack.last().unwrap();

    for f2 in neighbor_table.neighbor_list(f1) {
        let t2 = current_flip.next(*f2);
        check(
            depth,
            max_depth,
            distance,
            neighbor_table,
            current_flip,
            best_flip,
            edge_stack,
            gain,
            best_gain,
            selected,
            f1,
            t1,
            *f2,
            t2,
        );
    }

    for t2 in neighbor_table.neighbor_list(t1) {
        let f2 = current_flip.prev(*t2);
        check(
            depth,
            max_depth,
            distance,
            neighbor_table,
            current_flip,
            best_flip,
            edge_stack,
            gain,
            best_gain,
            selected,
            f1,
            t1,
            f2,
            *t2,
        );
    }
}

pub fn solve(distance: &(impl DistanceFunction + std::marker::Sync)) -> ArraySolution {
    let n = distance.dimension() as usize;
    // 解く
    // let solution = ArraySolution::new(distance.dimension() as usize);
    // let mut tlt = TwoLeveltreeSolution::<1000>::new(&solution);
    let mut tlt = ArraySolution::new(distance.dimension() as usize);

    let cache_filepath = PathBuf::from_str(format!("{}.cache", distance.name()).as_str()).unwrap();
    let neighbor_table = if cache_filepath.exists() {
        NeighborTable::load(&cache_filepath)
    } else {
        let table = NeighborTable::new(distance, 5);
        table.save(&cache_filepath);
        table
    };

    let mut rng = rand::thread_rng();

    let mut dlb = IntSet::new(n);
    dlb.set_all();

    let mut eval = evaluate(distance, &tlt);
    let mut selected = BitSet::new(n);

    let mut start = Instant::now();

    for iter in 0.. {
        let a = dlb.random_select(&mut rng);

        selected.clear_all();

        let diff = {
            let mut current_tree = SegmentTree::new(&tlt);
            let mut best_tree = SegmentTree::new(&tlt);

            let mut best_gain = 0;

            let a_next = tlt.next(a);
            let a_prev = tlt.prev(a);

            let mut edge_stack = vec![];

            // iterative deeping
            for max_depth in 2..=6 {
                for (a, b) in [(a_prev, a), (a, a_next)] {
                    selected.set(a);
                    selected.set(b);
                    edge_stack.push((a, b));

                    solve_inner(
                        1,
                        max_depth,
                        distance,
                        &neighbor_table,
                        &mut current_tree,
                        &mut best_tree,
                        &mut edge_stack,
                        0,
                        &mut best_gain,
                        &mut selected,
                    );

                    selected.clear(a);
                    selected.clear(b);
                    edge_stack.pop();
                }

                if best_gain > 0 {
                    break;
                }
            }
            if best_gain > 0 {
                Some((best_gain, best_tree.to_swap_list()))
            } else {
                None
            }
        };

        if let Some((gain, edge_list)) = diff {
            eval -= gain;
            for (from, to) in edge_list.into_iter() {
                tlt.swap(from, to);
                dlb.push(from);
                dlb.push(to);
            }
        } else {
            dlb.remove(a);
        }

        if iter % 100 == 0 {
            let end = Instant::now();
            let elapsed = (end - start).as_millis();
            if elapsed > 1000 {
                eprintln!("-----");
                eprintln!("iter: {}", iter);
                eprintln!("best eval: {}", eval);
                eprintln!("dlb size: {}", dlb.len());
                start = end;

                let exact_eval = evaluate(distance, &tlt);
                assert_eq!(exact_eval, eval);
            }
        }

        if dlb.is_empty() {
            break;
        }
    }
    tlt
}
