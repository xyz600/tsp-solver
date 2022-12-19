use rand::{rngs::ThreadRng, Rng};

pub struct IntSet {
    array: Vec<u32>,
    index_of: Vec<u32>,
    index: usize,
}

impl IntSet {
    const NONE: u32 = std::u32::MAX;

    pub fn new(n: usize) -> IntSet {
        IntSet {
            array: vec![0; n],
            index_of: vec![Self::NONE; n],
            index: 0,
        }
    }

    pub fn set_all(&mut self) {
        for i in 0..self.len() {
            self.push(i as u32);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub fn contains(&self, id: u32) -> bool {
        self.index_of[id as usize] == Self::NONE
    }

    pub fn push(&mut self, id: u32) {
        if !self.contains(id) {
            self.array[self.index] = id;
            self.index_of[id as usize] = self.index as u32;
            self.index += 1;
        }
    }

    pub fn remove(&mut self, id: u32) {
        assert!(self.index_of[id as usize] != Self::NONE);
        assert!(!self.is_empty());
        let id_index = self.index_of[id as usize];
        let last_id = self.array[self.index - 1];
        self.array[id_index as usize] = self.array[self.index - 1];
        self.index_of[last_id as usize] = id_index;
        self.index_of[id as usize] = Self::NONE;
        self.index -= 1;
    }

    pub fn random_select(&mut self, rng: &mut ThreadRng) -> u32 {
        assert!(!self.is_empty());
        rng.gen_range(0..self.index as u32)
    }

    pub fn len(&self) -> usize {
        self.index
    }
}
