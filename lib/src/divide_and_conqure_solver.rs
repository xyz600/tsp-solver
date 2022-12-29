use std::path::PathBuf;

use crate::{
    array_solution::ArraySolution,
    distance::DistanceFunction,
    lkh::{self, LKHConfig},
    solution::Solution,
};
use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

struct DividedDistance<'a, T: DistanceFunction> {
    ref_distance: &'a T,
    vertex_map: Vec<u32>,
    begin: u32,
    end: u32,
    name: String,
}

impl<'a, T: DistanceFunction> DividedDistance<'a, T> {
    fn new(
        ref_distance: &'a T,
        vertex_map: Vec<u32>,
        begin: u32,
        end: u32,
        name: String,
    ) -> DividedDistance<'a, T> {
        DividedDistance {
            ref_distance,
            vertex_map,
            begin,
            end,
            name,
        }
    }
}

impl<'a, T: DistanceFunction> DistanceFunction for DividedDistance<'a, T> {
    fn distance(&self, id1: u32, id2: u32) -> i64 {
        // 巡回路じゃなくて start -> end までのパスを求めたいので、ここを短絡させて経路を作る
        if (id1 == self.begin && id2 == self.end) || (id1 == self.end && id2 == self.begin) {
            0
        } else {
            let orig_id1 = self.vertex_map[id1 as usize];
            let orig_id2 = self.vertex_map[id2 as usize];
            self.ref_distance.distance(orig_id1, orig_id2)
        }
    }

    fn dimension(&self) -> u32 {
        self.vertex_map.len() as u32
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

// スレッド数で問題を分割して、最終的に統合
pub fn solve(
    distance: &(impl DistanceFunction + std::marker::Sync),
    solution: &impl Solution,
) -> ArraySolution {
    const NO_SPLIT: u32 = 12;

    let mut rng = thread_rng();
    let mut id = rng.gen_range(0..distance.dimension());
    let mut vertex_list = vec![vec![]; NO_SPLIT as usize];
    // 振り分け
    for no_segment in 0..NO_SPLIT {
        let segment_len = distance.dimension() * (no_segment + 1) / NO_SPLIT
            - distance.dimension() * no_segment / NO_SPLIT;
        for _iter in 0..segment_len {
            vertex_list[no_segment as usize].push(id);
            id = solution.next(id);
        }
    }

    // 分割統治の最適化
    let new_vertex_list = vertex_list.into_par_iter().map(|vertex_list| -> Vec<u32> {
        // 距離関数の生成
        // この順に元の解は [0, 1, 2, ..., ] という番号を付けるので、保存しておく
        let vertex_map = vertex_list.clone();
        let n = vertex_map.len() as u32 - 1;
        let partial_distance = DividedDistance::new(distance, vertex_list, 0, n, "".to_string());

        let init_solution = ArraySolution::new(partial_distance.dimension() as usize);
        let solution = lkh::solve(
            &partial_distance,
            init_solution,
            LKHConfig {
                use_neighbor_cache: false,
                cache_filepath: PathBuf::new(),
                debug: false,
                time_ms: 120_000,
                start_kick_step: 10,
                kick_step_diff: 10,
                end_kick_step: partial_distance.dimension() as usize / 10,
                fail_count_threashold: 50,
                max_depth: 6,
            },
        );

        // 分割の復元
        // solution 表記で 0 -> ... -> n の path を作りたいので、適切に方向を見て flip
        let in_order = solution.prev(0) == n;

        let mut vertex_array = vec![];
        let mut id = 0;
        for _iter in 0..solution.len() {
            let orig_id = vertex_map[id as usize];
            vertex_array.push(orig_id);
            id = if in_order {
                solution.next(id)
            } else {
                solution.prev(id)
            };
        }
        vertex_array
    });

    ArraySolution::from_array(new_vertex_list.flatten().collect::<Vec<_>>())
}
