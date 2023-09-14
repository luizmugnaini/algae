use crate::ds::MaxHeap;

// TODO: implement the `Sorter` trait for `HeapSort`.
pub struct HeapSort;

impl HeapSort {
    fn sort<T: PartialOrd + Copy>(xs: Vec<T>) -> Vec<T> {
        MaxHeap::heapsort(xs.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort;

    #[test]
    fn heapsort() {
        for _ in 0..50 {
            assert!(sort::is_sorted(&HeapSort::sort(sort::rand_vec(100))));
        }
    }
}
