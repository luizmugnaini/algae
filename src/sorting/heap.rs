use crate::ds::MaxHeap;

pub struct HeapSort;

impl HeapSort {
    fn sort<T: PartialOrd + Copy>(xs: Vec<T>) -> Vec<T> {
        MaxHeap::heapsort(xs.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting;

    #[test]
    fn heapsort() {
        assert!(sorting::is_sorted(&HeapSort::sort(sorting::rand_vec(100))));
    }
}
