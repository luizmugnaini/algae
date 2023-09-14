mod insertion;
pub use insertion::*;

mod merge;
pub use merge::*;

mod heap;
pub use heap::*;

use rand::{distributions::Uniform, Rng};
use std::cmp::PartialOrd;

trait Sorter {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]);
}

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
    let mut rng = rand::thread_rng();
    let range = Uniform::new(-1000, 1000);
    (0..vec_size).map(|_| rng.sample(&range)).collect()
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
