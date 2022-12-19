use crate::{
    array_solution::ArraySolution, distance::DistanceFunction,
    two_level_tree_solution::TwoLeveltreeSolution,
};

pub struct LinKernighanSolver {}

impl LinKernighanSolver {
    pub fn solve(distance: &impl DistanceFunction) -> ArraySolution {
        let solution = ArraySolution::new(distance.dimension() as usize);
        // 解く
        let mut two_level_tree_solution = TwoLeveltreeSolution::<1000>::new(&solution);

        solution
    }
}
