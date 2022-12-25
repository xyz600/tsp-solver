use std::time::Instant;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::distance::DistanceFunction;

pub struct NeighborTable {
    table: Vec<Vec<u32>>,
}

impl NeighborTable {
    pub fn new(
        distance: &(impl DistanceFunction + std::marker::Sync),
        neighbor_size: usize,
    ) -> NeighborTable {
        let n = distance.dimension();
        eprintln!("start to calculate neighbor table");
        let start = Instant::now();

        let table = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut distance_list = vec![];
                for j in 0..n {
                    if i != j {
                        distance_list.push((distance.distance(i, j), j));
                    }
                }
                distance_list.sort();
                distance_list
                    .iter()
                    .take(neighbor_size)
                    .map(|(_, index)| *index)
                    .collect()
            })
            .collect();
        eprintln!("finish to calculate neighbor table");
        let elapsed = (Instant::now() - start).as_millis();
        eprintln!("elapsed: {}[ms]", elapsed);

        NeighborTable { table }
    }

    pub fn neighbor_list(&self, id: u32) -> &Vec<u32> {
        &self.table[id as usize]
    }
}