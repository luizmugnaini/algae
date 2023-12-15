use crate::ds::MaxHeap;

pub fn heap_sort<T: PartialOrd + Copy>(xs: Vec<T>) -> Vec<T> {
    MaxHeap::heapsort(xs)
}

#[cfg(test)]
mod test {
    use crate::sort;

    #[test]
    fn heapsort() {
        for _ in 0..50 {
            assert!(sort::is_sorted(&super::heap_sort(sort::rand_vec(100))));
        }
    }
}
