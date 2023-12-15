mod insertion;
pub use insertion::*;

mod merge;
pub use merge::*;

mod heap;
pub use heap::*;

mod quick;
pub use quick::*;

use fastrand;
use std::{cmp::PartialOrd, iter};

pub fn is_sorted(xs: &[impl PartialOrd]) -> bool {
    let mut last = &xs[0];
    for next in xs {
        if last > next {
            return false;
        }
        last = next;
    }
    true
}

pub fn rand_vec(vec_size: usize) -> Vec<i64> {
    iter::repeat_with(|| fastrand::i64(..))
        .take(vec_size)
        .collect()
}

pub fn check_sort_fn<F: Fn(&mut [i64])>(sort_fn: F) {
    for _ in 0..50 {
        let mut xs = rand_vec(100);
        sort_fn(&mut xs);
        assert!(is_sorted(&xs));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted() {
        let v1 = vec![1, 2, 3, 4, 5, 5];
        assert!(is_sorted(&v1));

        let v2 = vec![0, 3, 9, 8, 10];
        assert!(!is_sorted(&v2));
    }
}
