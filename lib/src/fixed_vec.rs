pub trait FixedVecValue: Default + Clone + Copy {}

impl FixedVecValue for usize {}
impl FixedVecValue for isize {}

pub struct FixedVec<T, const N: usize> {
    array: [T; N],
    // 次に挿入する場所
    index: usize,
}

impl<T: FixedVecValue, const N: usize> FixedVec<T, N> {
    pub fn new() -> FixedVec<T, N> {
        FixedVec {
            array: [T::default(); N],
            index: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.array[self.index] = value;
        self.index += 1;
    }

    pub fn front(&self) -> T {
        assert!(!self.empty());
        self.array[0]
    }

    pub fn back(&self) -> T {
        assert!(!self.empty());
        self.array[self.index - 1]
    }

    pub fn pop_back(&mut self) {
        assert!(!self.empty());
        self.index -= 1;
    }

    pub fn empty(&self) -> bool {
        self.index == 0
    }

    pub fn len(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::FixedVec;

    #[test]
    fn test_fixed_vector() {
        const N: usize = 128;
        let mut v = FixedVec::<usize, N>::new();
        assert!(v.empty());

        let size = 16;
        for i in 0..size {
            v.push(i);
        }
        assert_eq!(v.front(), 0);
        assert_eq!(v.back(), size - 1);
        assert_eq!(v.len(), size);
    }
}
