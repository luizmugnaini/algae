/// Merge sort algorithm.
pub fn merge_sort<T: PartialOrd + Copy>(xs: &mut [T]) {
    merge_sort_rec(xs, 0, xs.len() - 1);
}

/// Recursive counterpart of merge-sort.
fn merge_sort_rec<T: PartialOrd + Copy>(xs: &mut [T], low: usize, top: usize) {
    if low < top {
        let mid = (low + top) / 2;
        merge_sort_rec(xs, low, mid);
        merge_sort_rec(xs, mid + 1, top);
        merge(xs, low, mid, top);
    }
}

/// Merges two sorted arrays into the original array `xs`.
/// * `low`: index of the first element.
/// * `mid`: index of middle element.
/// * `top`: index of the last element.
///
/// Takes time `O(n)` where `n = top - low + 1`.
fn merge<T: PartialOrd + Copy>(xs: &mut [T], low: usize, mid: usize, top: usize) {
    let left = xs[low..=mid].to_vec();
    let right = xs[(mid + 1)..=top].to_vec();

    // Merge `left` and `right` into `xs`
    xs[low..=top].iter_mut().fold((0, 0), |(i, j), x| {
        let in_left = i < left.len();
        let in_right = j < right.len();

        if in_left && in_right {
            // Check for the minimum element
            if left[i] <= right[j] {
                *x = left[i];
                (i + 1, j)
            } else {
                *x = right[j];
                (i, j + 1)
            }
        } else if in_left {
            // Dump the remainder of the left array
            *x = left[i];
            (i + 1, j)
        } else {
            // Dump the remainder of the right array
            *x = right[j];
            (i, j + 1)
        }
    });
}

#[cfg(test)]
mod test {
    use crate::sort;

    #[test]
    fn sorting() {
        sort::check_sort_fn(super::merge_sort);
    }

    #[test]
    fn merge_arrays() {
        let mut xs = vec![5, 6, 7, 8, 1, 2, 3, 4];
        super::merge(&mut xs, 0, 3, 7);
        assert_eq!(xs, vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let mut xs = vec![13, 11, 5, 6, 7, 8, 1, 2, 3, 4, 90, 21];
        super::merge(&mut xs, 2, 5, 9);
        assert_eq!(xs, vec![13, 11, 1, 2, 3, 4, 5, 6, 7, 8, 90, 21]);
    }
}
