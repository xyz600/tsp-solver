use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

use crate::solution::Solution;

#[derive(Clone)]
struct Segment<const N: usize> {
    array: [u32; N],
    // 次に挿入する場所
    index: usize,
    reversed: bool,
}

impl<const N: usize> Segment<N> {
    pub fn new() -> Segment<N> {
        Segment {
            array: [0u32; N],
            index: 0,
            reversed: false,
        }
    }

    pub fn front(&self) -> u32 {
        if self.reversed {
            self.array[self.len() - 1]
        } else {
            self.array[0]
        }
    }

    pub fn back(&self) -> u32 {
        if self.reversed {
            self.array[0]
        } else {
            self.array[self.len() - 1]
        }
    }

    pub fn clear(&mut self) {
        self.reversed = false;
        self.index = 0;
    }

    pub fn push(&mut self, value: u32) {
        self.array[self.index] = value;
        self.index += 1;
    }

    pub fn pop(&mut self) {
        assert!(!self.is_empty());
        self.index -= 1;
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub fn len(&self) -> usize {
        self.index
    }

    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        self.array.swap(idx1, idx2);
    }
}

impl<const N: usize> Index<usize> for Segment<N> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<const N: usize> IndexMut<usize> for Segment<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

struct SegmentIDList {
    content: Vec<u16>,
    index_of: Vec<u16>,
    free_list: Vec<u16>,
}

impl SegmentIDList {
    const NONE: u16 = std::u16::MAX;
    fn new(n: u16) -> SegmentIDList {
        SegmentIDList {
            content: vec![],
            index_of: vec![SegmentIDList::NONE; n as usize],
            // stack として pop した時に、小さい数字が使われる傾向にあれば見やすい
            free_list: (0..n).rev().collect(),
        }
    }

    fn acquire_free_segment_id(&mut self) -> u16 {
        assert!(!self.free_list.is_empty());
        self.free_list.pop().unwrap()
    }

    fn contains(&self, segment_id: u16) -> bool {
        self.index_of[segment_id as usize] != SegmentIDList::NONE
    }

    fn push(&mut self, segment_id: u16) {
        assert!(!self.contains(segment_id));
        self.index_of[segment_id as usize] = self.content.len() as u16;
        self.content.push(segment_id);
    }

    fn remove(&mut self, segment_id: u16) {
        assert!(self.contains(segment_id));
        let remove_pos = self.index_of[segment_id as usize] as usize;
        self.content.remove(remove_pos);
        self.index_of[segment_id as usize] = SegmentIDList::NONE;
        for index in remove_pos..self.content.len() {
            let id = self.content[index];
            assert!(self.index_of[id as usize] >= 1);
            self.index_of[id as usize] -= 1;
        }
        self.free_list.push(segment_id);
    }

    // target_segment_id の前に segment_id を挿入する
    fn insert_prev(&mut self, segment_id: u16, target_segment_id: u16) {
        assert!(self.contains(target_segment_id));
        assert!(!self.contains(segment_id));
        let insert_pos = self.index_of[target_segment_id as usize] as usize;
        self.content.insert(insert_pos, segment_id);
        for index in insert_pos..self.content.len() {
            let id = self.content[index];
            self.index_of[id as usize] = index as u16;
        }
    }

    // target_segment_id の次に segment_id を挿入する
    fn insert_next(&mut self, segment_id: u16, target_segment_id: u16) {
        assert!(self.contains(target_segment_id));
        assert!(!self.contains(segment_id));
        if self.index_of[target_segment_id as usize] == self.len() as u16 - 1 {
            self.push(segment_id);
        } else {
            // 末尾以外は、insert_next(p, t) = insert_prev(p, next(t))
            let prev_target_index = self.index_of[target_segment_id as usize] as usize + 1;
            let prev_target_id = self.content[prev_target_index];
            self.insert_prev(segment_id, prev_target_id);
        }
    }

    fn next(&self, id: u16) -> u16 {
        assert!(self.contains(id));
        let index = self.index_of[id as usize] as usize;
        if index == self.len() - 1 {
            self.content[0]
        } else {
            self.content[index + 1]
        }
    }

    fn prev(&self, id: u16) -> u16 {
        assert!(self.contains(id));
        let index = self.index_of[id as usize] as usize;
        if index == 0 {
            self.content[self.len() - 1]
        } else {
            self.content[index - 1]
        }
    }

    fn swap(&mut self, from: u16, to: u16) {
        let mut from_index = self.index_of[from as usize] as usize;
        let mut to_index = self.index_of[to as usize] as usize;

        let range_size = if from_index <= to_index {
            to_index + 1 - from_index
        } else {
            to_index + 1 + self.len() - from_index
        };

        for _iter in 0..(range_size / 2) {
            let from = self.content[from_index] as usize;
            let to = self.content[to_index] as usize;

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

    fn index_of(&self, id: u16) -> u16 {
        self.index_of[id as usize]
    }

    fn len(&self) -> usize {
        self.content.len()
    }
}

#[derive(Clone, Copy, Default)]
struct TwoLevelIndex {
    segment_id: u16,
    inner_id: u16,
}

pub struct TwoLeveltreeSolution<const N: usize> {
    // segment の中身
    buffer: Vec<Segment<N>>,
    // 各都市の情報
    index_of: Vec<TwoLevelIndex>,
    // 格納されている segment の順番
    segment_list: SegmentIDList,
}

impl<const N: usize> TwoLeveltreeSolution<N> {
    pub fn new<T: Solution>(sol: &T) -> TwoLeveltreeSolution<N> {
        let len = sol.len();
        let mut index_of = vec![TwoLevelIndex::default(); len];

        // \sqrt{N} 個くらいの segment に分割して登録
        let segment_size = (len as f64).sqrt().ceil() as usize;
        let segment_capacity = segment_size * 2;
        let mut segment_list = SegmentIDList::new(segment_capacity as u16);
        let mut buffer = vec![Segment::<N>::new(); segment_capacity];

        let mut node = 0;
        for iter in 0..segment_size {
            let segment_id = segment_list.acquire_free_segment_id();
            let segment_size = len * (iter + 1) / segment_size - len * iter / segment_size;
            assert!(segment_size < N);
            for inner_id in 0..segment_size as u16 {
                buffer[segment_id as usize].push(node);
                index_of[node as usize].inner_id = inner_id;
                index_of[node as usize].segment_id = segment_id;
                node = sol.next(node as u32);
            }
            segment_list.push(segment_id as u16);
        }

        TwoLeveltreeSolution {
            buffer,
            index_of,
            segment_list,
        }
    }

    // 物理的な配置について、登場順序順に比較する
    fn cmp(&self, id1: u32, id2: u32) -> Ordering {
        let index1 = self
            .segment_list
            .index_of(self.index_of[id1 as usize].segment_id);
        let inner_index1 = self.index_of[id1 as usize].inner_id;
        let index2 = self
            .segment_list
            .index_of(self.index_of[id2 as usize].segment_id);
        let inner_index2 = self.index_of[id2 as usize].inner_id;

        let pair_index1 = (index1, inner_index1);
        let pair_index2 = (index2, inner_index2);
        pair_index1.cmp(&pair_index2)
    }

    // [prev, id, next, ...] -> [..., prev] + [id, next, ...] として、分割された segment id の pair
    fn split(&mut self, id: u32) -> (u16, u16) {
        let index = self.index_of[id as usize];
        let segment_id = index.segment_id;
        let new_segment_id = self.segment_list.acquire_free_segment_id();

        // 物理的に front / back にあるという意味であることに注意
        let front_segment_size = index.inner_id;
        let all_segment_size = self.buffer[segment_id as usize].len() as u16;
        let back_segment_size = all_segment_size - front_segment_size;

        // buffer の先頭を書き換えないように、どうにか頑張る
        self.buffer[new_segment_id as usize].reversed = self.buffer[segment_id as usize].reversed;
        for idx in front_segment_size..all_segment_size {
            let id = self.buffer[segment_id as usize][idx as usize];
            self.buffer[new_segment_id as usize][(idx - front_segment_size) as usize] = id;
            self.index_of[id as usize].segment_id = new_segment_id;
            self.index_of[id as usize].inner_id = idx - front_segment_size;
        }
        self.buffer[segment_id as usize].index = front_segment_size as usize;
        self.buffer[new_segment_id as usize].index = back_segment_size as usize;

        if self.buffer[segment_id as usize].reversed {
            // buffer の後半 = sequence の前半なので、(new, cur) の segment 順序になる
            self.segment_list.insert_prev(new_segment_id, segment_id);
            (new_segment_id, segment_id)
        } else {
            // buffer の前半 = sequence の前半なので、(cur, new) の segment 順序になる
            self.segment_list.insert_next(new_segment_id, segment_id);
            (segment_id, new_segment_id)
        }
    }

    fn dissolve_reverse(&mut self, segment_id: u16) {
        let segment_id = segment_id as usize;
        assert!(self.buffer[segment_id].reversed);
        self.buffer[segment_id].reversed = false;

        let len = self.buffer[segment_id].len();
        let max_iter = self.buffer[segment_id].len() / 2;
        for i in 0..max_iter {
            self.buffer[segment_id].swap(i, len - 1 - i);
            let id1 = self.buffer[segment_id][i] as usize;
            let id2 = self.buffer[segment_id][len - 1 - i] as usize;
            self.index_of.swap(id1, id2);
        }
    }

    // (segment_id, next_segment_id) を merge して、残った segment_id を返す
    fn merge_right(&mut self, segment_id: u16) -> u16 {
        // [segment_id, next_segment_id] を merge する
        let next_segment_id = self.segment_list.next(segment_id);

        // merge 元と merge 先の reverse の状態が合っていないと辛いので、不ぞろいなら揃える
        if self.buffer[segment_id as usize].reversed
            != self.buffer[next_segment_id as usize].reversed
        {
            if self.buffer[segment_id as usize].reversed {
                self.dissolve_reverse(segment_id);
            } else {
                self.dissolve_reverse(next_segment_id);
            }
        }

        let segment_id = segment_id as usize;
        let next_segment_id = next_segment_id as usize;

        if self.buffer[segment_id].reversed {
            // next に current を詰める
            let offset = self.buffer[next_segment_id].len();
            for i in 0..self.buffer[segment_id].len() {
                let id = self.buffer[segment_id][i];
                self.buffer[next_segment_id][i + offset] = id;
                self.index_of[id as usize].segment_id = next_segment_id as u16;
                self.index_of[id as usize].inner_id = (i + offset) as u16;
            }
            self.segment_list.remove(segment_id as u16);
            next_segment_id as u16
        } else {
            // current に next を詰める
            let offset = self.buffer[segment_id].len();
            for i in 0..self.buffer[next_segment_id].len() {
                let id = self.buffer[next_segment_id][i];
                self.buffer[segment_id][i + offset] = id;
                self.index_of[id as usize].segment_id = segment_id as u16;
                self.index_of[id as usize].inner_id = (i + offset) as u16;
            }
            self.segment_list.remove(next_segment_id as u16);
            segment_id as u16
        }
    }

    // 単一セグメントのみの swap
    fn swap_in_segment(&mut self, from: u32, to: u32) {
        let from_index = self.index_of[from as usize];
        let to_index = self.index_of[to as usize];
        assert_eq!(from_index.segment_id, to_index.segment_id);
        let segment_id = from_index.segment_id;

        let (mut from_idx, mut to_idx) = if self.buffer[segment_id as usize].reversed {
            (to_index.inner_id, from_index.inner_id)
        } else {
            (from_index.inner_id, to_index.inner_id)
        };
        // Segment 内で環状に接続されているわけではないため
        assert!(from_idx <= to_idx);

        let len = to_index.inner_id + 1 - from_index.inner_id;
        for _iter in 0..len / 2 {
            self.buffer[segment_id as usize].swap(from_idx as usize, to_idx as usize);
            let from = self.buffer[segment_id as usize][from_idx as usize];
            let to = self.buffer[segment_id as usize][to_idx as usize];
            self.index_of.swap(from as usize, to as usize);
            from_idx += 1;
            to_idx -= 1;
        }
    }

    // セグメント単位でのみswap
    fn swap_aligned(&mut self, from_segment_id: u16, to_segment_id: u16) {
        let mut segment_id = from_segment_id;
        while segment_id != to_segment_id {
            self.buffer[segment_id as usize].reversed ^= true;
            segment_id = self.segment_list.next(segment_id);
        }
        self.buffer[to_segment_id as usize].reversed ^= true;

        self.segment_list.swap(from_segment_id, to_segment_id);
    }
}

impl<const N: usize> Solution for TwoLeveltreeSolution<N> {
    fn prev(&self, id: u32) -> u32 {
        let index = self.index_of[id as usize];
        if self.buffer[index.segment_id as usize].reversed {
            if index.inner_id == self.buffer[index.segment_id as usize].len() as u16 - 1 {
                let prev_segment_id = self.segment_list.prev(index.segment_id);
                self.buffer[prev_segment_id as usize].back()
            } else {
                self.buffer[index.segment_id as usize][index.inner_id as usize + 1]
            }
        } else {
            if index.inner_id == 0 {
                let prev_segment_id = self.segment_list.prev(index.segment_id);
                self.buffer[prev_segment_id as usize].back()
            } else {
                self.buffer[index.segment_id as usize][index.inner_id as usize - 1]
            }
        }
    }

    fn next(&self, id: u32) -> u32 {
        let index = self.index_of[id as usize];
        if self.buffer[index.segment_id as usize].reversed {
            if index.inner_id == 0 {
                let next_segment_id = self.segment_list.next(index.segment_id);
                self.buffer[next_segment_id as usize].front()
            } else {
                self.buffer[index.segment_id as usize][index.inner_id as usize - 1]
            }
        } else {
            if index.inner_id == self.buffer[index.segment_id as usize].len() as u16 - 1 {
                let next_segment_id = self.segment_list.next(index.segment_id);
                self.buffer[next_segment_id as usize].front()
            } else {
                self.buffer[index.segment_id as usize][index.inner_id as usize + 1]
            }
        }
    }

    fn between(&self, id: u32, from: u32, to: u32) -> bool {
        match self.cmp(from, to) {
            Ordering::Less => {
                // from <= id <= to
                self.cmp(from, id) != Ordering::Greater && self.cmp(id, to) != Ordering::Greater
            }
            Ordering::Equal => id == from,
            Ordering::Greater => {
                // (to <= from <= id) or
                // (id <= to <= from)
                self.cmp(from, id) != Ordering::Less || self.cmp(id, to) != Ordering::Less
            }
        }
    }

    fn swap(&mut self, from: u32, to: u32) {
        let from_segment = self.index_of[from as usize].segment_id;
        let to_segment = self.index_of[to as usize].segment_id;

        if from_segment == to_segment {
            self.swap_in_segment(from, to);
        } else {
            let from_segment = if from == self.buffer[from_segment as usize].front() {
                from_segment
            } else {
                let (_, new_from_segment) = self.split(from);
                new_from_segment
            };

            let to_segment = if to == self.buffer[to_segment as usize].back() {
                to_segment
            } else {
                // [..., prev, to, next, ...] -> [..., prev, to], [next, ...]
                let (new_to_segment, _) = self.split(self.next(to));
                new_to_segment
            };

            self.swap_aligned(from_segment, to_segment);

            // [..., prev_from, from_segment, ..., to_segment, next_to, ...]
            // -> [..., prev_from, to_segment, ..., from_segment, next_to, ...]
            // from, to の segment size が小さすぎたら、それぞれ merge
            let merge_threashold = (self.len() as f64).sqrt().ceil() as usize / 2;
            for segment_id in [from_segment, to_segment] {
                if self.buffer[segment_id as usize].len() < merge_threashold {
                    let prev = self.segment_list.prev(segment_id);
                    let prev_len = self.buffer[prev as usize].len();
                    let next = self.segment_list.next(segment_id);
                    let next_len = self.buffer[next as usize].len();
                    if prev_len < next_len {
                        if prev_len < merge_threashold {
                            self.merge_right(prev);
                        }
                    } else {
                        if next_len < merge_threashold {
                            self.merge_right(segment_id);
                        }
                    }
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.index_of.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{array_solution::ArraySolution, solution::Solution};

    use super::{SegmentIDList, TwoLeveltreeSolution};

    use rand::Rng;

    #[test]
    fn test_segment_list() {
        let segment_size = 64;
        let segment_capacity = segment_size * 2;
        let mut segment_list = SegmentIDList::new(segment_capacity);

        const SIZE: u16 = 20;
        for i in 0..SIZE {
            segment_list.push(i);
        }
        for i in 0..SIZE {
            let expected = (i + 1) % SIZE;
            assert_eq!(segment_list.next(i), expected);

            let expected = (i + SIZE - 1) % SIZE;
            assert_eq!(segment_list.prev(i), expected);
        }

        // insert_next
        segment_list.insert_next(SIZE, 10);
        assert_eq!(segment_list.prev(10), 9);
        assert_eq!(segment_list.next(10), SIZE);
        assert_eq!(segment_list.prev(SIZE), 10);
        assert_eq!(segment_list.next(SIZE), 11);
        assert_eq!(segment_list.prev(11), SIZE);
        assert_eq!(segment_list.next(11), 12);

        // remove
        segment_list.remove(SIZE);
        for i in 0..SIZE {
            let expected = (i + 1) % SIZE;
            assert_eq!(segment_list.next(i), expected);

            let expected = (i + SIZE - 1) % SIZE;
            assert_eq!(segment_list.prev(i), expected);
        }

        // insert prev
        segment_list.insert_prev(SIZE, 11);
        assert_eq!(segment_list.prev(10), 9);
        assert_eq!(segment_list.next(10), SIZE);
        assert_eq!(segment_list.prev(SIZE), 10);
        assert_eq!(segment_list.next(SIZE), 11);
        assert_eq!(segment_list.prev(11), SIZE);
        assert_eq!(segment_list.next(11), 12);
    }

    fn check(from: u32, to: u32) {
        const SIZE: usize = 100;
        let solution = ArraySolution::new(SIZE);
        let mut two_level_tree = TwoLeveltreeSolution::<30>::new(&solution);

        for i in 0..SIZE {
            let expected = (i + 1) % SIZE;
            assert_eq!(two_level_tree.next(i as u32), expected as u32);

            let expected = (i + SIZE - 1) % SIZE;
            assert_eq!(two_level_tree.prev(i as u32), expected as u32);
        }

        two_level_tree.swap(from, to);
        assert_eq!(two_level_tree.prev(from - 1), from - 2);
        assert_eq!(two_level_tree.next(from - 1), to);
        assert_eq!(two_level_tree.prev(to), from - 1);
        assert_eq!(two_level_tree.next(to), to - 1);
        assert_eq!(two_level_tree.prev(from), from + 1);
        assert_eq!(two_level_tree.next(from), to + 1);
        assert_eq!(two_level_tree.prev(to + 1), from);
        assert_eq!(two_level_tree.next(to + 1), to + 2);
    }

    #[test]
    fn test_two_level_tree_case1() {
        check(37, 94);
    }

    #[test]
    fn test_two_level_tree_case2() {
        check(20, 80);
    }

    #[test]
    fn test_two_level_tree_case3() {
        check(17, 92);
    }

    #[test]
    fn test_two_level_tree_case4() {
        const SIZE: usize = 100;
        let solution = ArraySolution::new(SIZE);
        let mut two_level_tree = TwoLeveltreeSolution::<30>::new(&solution);

        for i in 0..SIZE {
            let expected = (i + 1) % SIZE;
            assert_eq!(two_level_tree.next(i as u32), expected as u32);

            let expected = (i + SIZE - 1) % SIZE;
            assert_eq!(two_level_tree.prev(i as u32), expected as u32);
        }

        // [16, 17, 18, 19] -> [16, 18, 17, 19]
        two_level_tree.swap(17, 18);
        assert_eq!(two_level_tree.prev(16), 15);
        assert_eq!(two_level_tree.next(16), 18);
        assert_eq!(two_level_tree.prev(18), 16);
        assert_eq!(two_level_tree.next(18), 17);
        assert_eq!(two_level_tree.prev(17), 18);
        assert_eq!(two_level_tree.next(17), 19);
        assert_eq!(two_level_tree.prev(19), 17);
        assert_eq!(two_level_tree.next(19), 20);
    }

    #[test]
    fn test_two_level_tree_case5() {
        check(10, 24);
    }

    #[test]
    fn test_two_level_tree_case6() {
        const SIZE: usize = 100;

        let mut solution = ArraySolution::new(SIZE);
        let mut two_level_tree = TwoLeveltreeSolution::<30>::new(&solution);

        for (from, to) in [(91, 47), (10, 98)] {
            solution.swap(from, to);
            two_level_tree.swap(from, to);

            // check
            let mut id = 0;
            for _iter in 0..SIZE {
                let next_sol = solution.next(id);
                let next_two_level_tree = two_level_tree.next(id);
                eprintln!("id = {}", id);
                eprintln!(
                    "    [prev] sol: {}, tlt: {}",
                    solution.prev(id),
                    two_level_tree.prev(id)
                );
                eprintln!("    [next] sol: {}, tlt: {}", next_sol, next_two_level_tree);
                assert_eq!(next_sol, next_two_level_tree);
                assert_eq!(solution.prev(id), two_level_tree.prev(id));
                id = next_sol;
            }
        }
    }

    #[test]
    fn test_two_level_tree_random() {
        const SIZE: usize = 100;

        let mut solution = ArraySolution::new(SIZE);
        let mut two_level_tree = TwoLeveltreeSolution::<30>::new(&solution);

        let mut rng = rand::thread_rng();

        for _iter in 0..2000 {
            eprintln!("{}", _iter);
            let from = rng.gen_range(0..SIZE as u32);
            let to = rng.gen_range(0..SIZE as u32);
            if from != to {
                eprintln!("from = {}, to = {}", from, to);

                solution.swap(from, to);
                two_level_tree.swap(from, to);

                // check
                let mut id = 0;
                for _iter in 0..SIZE {
                    let next_sol = solution.next(id);
                    let next_two_level_tree = two_level_tree.next(id);
                    eprintln!("id = {}", id);
                    eprintln!(
                        "    [prev] sol: {}, tlt: {}",
                        solution.prev(id),
                        two_level_tree.prev(id)
                    );
                    eprintln!("    [next] sol: {}, tlt: {}", next_sol, next_two_level_tree);
                    assert_eq!(next_sol, next_two_level_tree);
                    assert_eq!(solution.prev(id), two_level_tree.prev(id));
                    id = next_sol;
                }
            }
        }
    }
}
