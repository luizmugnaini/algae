use super::{InsertionSort, Sorter};
use rand::seq::SliceRandom;
use std::cmp::Ordering;

/// Cutoff to insertion sort: since quicksort is a recursive algorithm, its
/// performance is worse than insertion sort for tiny subarrays. This leads way
/// to defining a cutoff constant to be used. If `high` and `low` are the
/// highest and lowest index of a subarray, then for `high <= low + CUTOFF` we
/// jump from quicksort to insertion sort.
const CUTOFF: usize = 10;

/// Quicksort algorithm. The implementation also uses the cutoff method for
/// small arrays in order to reduce the number of calls in the stack generated
/// by the recursive nature of the quicksort algorithm. Instead of applying
/// quicksort, for these small arrays we apply the straight forward
/// insertion sort.
///
/// Example:
/// ```
/// use algorithmia::sort::{QuickSort, Sorter};
///
/// let mut v = [99, 32, 58, 66, 2, 4, 0, 3928, 55, 88, 30, 44, 3, 2, 0];
/// QuickSort::sort(&mut v);
/// assert_eq!(v, [0, 0, 2, 2, 3, 4, 30, 32, 44, 55, 58, 66, 88, 99, 3928]);
/// ```
pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        let mut rng = rand::thread_rng();
        xs.shuffle(&mut rng);
        Self::_sort(xs, 0, xs.len().checked_sub(1).unwrap_or(0));
    }
}

impl QuickSort {
    fn _sort<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) {
        if high <= low + CUTOFF {
            InsertionSort::sort(xs);
            return;
        }
        let pivot_idx = Self::partition(xs, low, high);
        Self::_sort(xs, low, pivot_idx.checked_sub(1).unwrap_or(0));
        Self::_sort(xs, pivot_idx + 1, high);
    }

    fn partition<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) -> usize {
        let pivot = xs[low];
        let mut lscan = low + 1;
        let mut rscan = high;
        loop {
            while xs[lscan] <= pivot {
                if lscan == high {
                    break;
                }
                lscan += 1;
            }

            while pivot <= xs[rscan] {
                if rscan == low {
                    break;
                }
                rscan = rscan.checked_sub(1).unwrap_or(0);
            }

            if rscan <= lscan {
                break;
            }
            xs.swap(lscan, rscan);
        }
        xs.swap(low, rscan);
        return rscan;
    }
}

/// The `Quick3WaySort` implements the quicksort algorithm via Dijkstra's
/// approach. The method is based on a single left-to-right pass through the
/// array. The goal is to divide the array into three parts. Let `v` be an array
/// and `i` be a pivot index of `v`:
/// * We construct to the left of `i` the subarray of elements that are strictly
///   less than `v[i]`.
/// * To the right of `i` we construct the subarray of elements equal to `v[i]`.
/// * To the right of the latter subarray we construct a subarray composed of
///   all elements strictly greater than `v[i]`.
///
/// For performance concens, we also apply the cutoff method: that is, for small
/// enough subarrays, we abandon the quicksort algorithm and run insertion sort.
/// This increases performance due to the fact that subarrays would populate the
/// call stack unnecessarily due to the fact that quicksort is a recursive
/// algorithm.
///
/// Example:
/// ```
/// use algorithmia::sort::{Quick3WaySort, Sorter};
///
/// let mut v = "quicksortexample".as_bytes().to_vec();
/// Quick3WaySort::sort(&mut v);
/// assert_eq!("aceeiklmopqrstux", &String::from_utf8(v).unwrap());
/// ```
pub struct Quick3WaySort;

impl Sorter for Quick3WaySort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        let mut rng = rand::thread_rng();
        xs.shuffle(&mut rng);
        Self::_sort(xs, 0, xs.len().checked_sub(1).unwrap_or(0));
    }
}

impl Quick3WaySort {
    fn _sort<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) {
        if high <= low + CUTOFF {
            InsertionSort::sort(xs);
            return;
        }

        let mut lt = low;
        let mut gt = high;
        let mut scan = low + 1;

        while scan <= gt {
            match xs[scan]
                .partial_cmp(&xs[low])
                .expect("Unable to compare values")
            {
                Ordering::Less => {
                    xs.swap(lt, scan);
                    lt += 1;
                    scan += 1;
                }
                Ordering::Greater => {
                    xs.swap(scan, gt);
                    gt = gt.checked_sub(1).unwrap_or(0);
                }
                Ordering::Equal => scan += 1,
            }
        }
        Self::_sort(xs, low, lt.checked_sub(1).unwrap_or(0));
        Self::_sort(xs, gt + 1, high);
    }
}

#[cfg(test)]
mod test {
    use super::{Quick3WaySort, QuickSort};
    use crate::sort;

    #[test]
    fn sorting_quicksort() {
        sort::check_sorter(QuickSort);
    }

    #[test]
    fn sorting_quick3waysort() {
        sort::check_sorter(Quick3WaySort);
    }
}
