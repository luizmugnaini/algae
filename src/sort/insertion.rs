use super::Sorter;
use std::cmp::PartialOrd;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        for not_sorted in 1..xs.len() {
            let mut i = not_sorted;
            while i > 0 && xs[i - 1] > xs[i] {
                xs.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort;

    #[test]
    fn insertion() {
        let mut xs = sort::rand_vec(100);
        InsertionSort::sort(&mut xs);
        assert!(sort::is_sorted(&xs));
    }
}
