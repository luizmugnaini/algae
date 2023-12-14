use std::cmp::PartialOrd;

pub fn insertion_sort<T: PartialOrd + Copy>(xs: &mut [T]) {
    for not_sorted in 1..xs.len() {
        let mut i = not_sorted;
        while i > 0 && xs[i - 1] > xs[i] {
            xs.swap(i - 1, i);
            i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sort;

    #[test]
    fn insertion_sort_test() {
        sort::check_sort_fn(super::insertion_sort);
    }
}
