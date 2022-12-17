pub trait Solution {
    fn prev(&self, id: usize) -> usize;

    fn next(&self, id: usize) -> usize;

    // whether id is in [from, to]
    fn between(&self, id: usize, from: usize, to: usize) -> bool;

    // swap range [from, to]
    fn swap(&mut self, from: usize, to: usize);
}
