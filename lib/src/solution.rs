pub trait Solution {
    fn prev(&self, id: u32) -> u32;

    fn next(&self, id: u32) -> u32;

    // whether id is in [from, to]
    fn between(&self, id: u32, from: u32, to: u32) -> bool;

    // swap range [from, to]
    fn swap(&mut self, from: u32, to: u32);

    fn len(&self) -> usize;
}
