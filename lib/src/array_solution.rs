use crate::solution::Solution;

pub struct ArraySolution {
    content: Vec<usize>,
    index_of: Vec<usize>,
}

impl ArraySolution {
    pub fn new(dimension: usize) -> ArraySolution {
        ArraySolution {
            content: (0..dimension).collect(),
            index_of: (0..dimension).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

impl Solution for ArraySolution {
    fn prev(&self, id: usize) -> usize {
        let index = self.index_of[id];
        if index == 0 {
            self.content[self.len() - 1]
        } else {
            self.content[index - 1]
        }
    }

    fn next(&self, id: usize) -> usize {
        let index = self.index_of[id];
        if index == self.len() - 1 {
            self.content[0]
        } else {
            self.content[index + 1]
        }
    }

    fn between(&self, id: usize, from: usize, to: usize) -> bool {
        let id_index = self.index_of[id];
        let from_index = self.index_of[from];
        let to_index = self.index_of[to];
        if from_index <= to_index {
            from_index <= id_index && id_index <= to_index
        } else {
            // id -> to -> from の順になっていれば ok
            id_index <= to_index && to_index <= from_index
        }
    }

    fn swap(&mut self, from: usize, to: usize) {
        let mut from_index = self.index_of[from];
        let mut to_index = self.index_of[to];

        let range_size = if from_index <= to_index {
            to_index - from_index
        } else {
            to_index + self.len() - from_index
        };

        for _iter in 0..(range_size / 2) {
            let from = self.content[from_index];
            let to = self.content[to_index];
            self.index_of.swap(from, to);
            self.content.swap(from_index, to_index);
            from_index = if from_index == self.len() - 1 {
                0
            } else {
                from_index + 1
            };
            to_index = if to_index == 0 {
                self.len() - 1
            } else {
                to_index - 1
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    use super::ArraySolution;

    #[test]
    fn test_solution_swap() {
        let dimension = 100;
        let mut solution = ArraySolution::new(dimension);

        // [0, ..., 19, 20, 21, ..., 79, 80, 81, ...]
        assert_eq!(solution.prev(19), 18);
        assert_eq!(solution.next(19), 20);
        assert_eq!(solution.prev(80), 79);
        assert_eq!(solution.next(80), 81);
        assert_eq!(solution.prev(20), 19);
        assert_eq!(solution.next(20), 21);
        assert_eq!(solution.prev(81), 80);
        assert_eq!(solution.next(81), 82);

        solution.swap(20, 80);
        // [0, ..., 19, 80, 79, ..., 21, 20, 81, ...]
        assert_eq!(solution.prev(19), 18);
        assert_eq!(solution.next(19), 80);
        assert_eq!(solution.prev(80), 19);
        assert_eq!(solution.next(80), 79);
        assert_eq!(solution.prev(20), 21);
        assert_eq!(solution.next(20), 81);
        assert_eq!(solution.prev(81), 20);
        assert_eq!(solution.next(81), 82);
    }

    #[test]
    fn test_solution_swap2() {
        let dimension = 100;
        let mut solution = ArraySolution::new(dimension);

        solution.swap(80, 20);
        // [0, 99, 98, ..., 81, 80, 21, 22, ..., 79, 20, 19, ..., 1]
        assert_eq!(solution.prev(19), 20);
        assert_eq!(solution.next(19), 18);
        assert_eq!(solution.prev(80), 81);
        assert_eq!(solution.next(80), 21);
        assert_eq!(solution.prev(20), 79);
        assert_eq!(solution.next(20), 19);
        assert_eq!(solution.prev(81), 82);
        assert_eq!(solution.next(81), 80);
    }
}