use crate::sort;
use fastrand;
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
/// use algae::sort;
///
/// let mut v = [99, 32, 58, 66, 2, 4, 0, 3928, 55, 88, 30, 44, 3, 2, 0];
/// sort::quick_sort(&mut v);
/// assert_eq!(v, [0, 0, 2, 2, 3, 4, 30, 32, 44, 55, 58, 66, 88, 99, 3928]);
/// ```
pub fn quick_sort<T: PartialOrd + Copy>(xs: &mut [T]) {
    fastrand::shuffle(xs);
    quick_sort_rec(xs, 0, xs.len().saturating_sub(1));
}

fn quick_sort_rec<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) {
    if high <= low + CUTOFF {
        sort::insertion_sort(xs);
        return;
    }
    let pivot_idx = quick_sort_partition(xs, low, high);
    quick_sort_rec(xs, low, pivot_idx.saturating_sub(1));
    quick_sort_rec(xs, pivot_idx + 1, high);
}

fn quick_sort_partition<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) -> usize {
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
            rscan = rscan.saturating_sub(1);
        }

        if rscan <= lscan {
            break;
        }
        xs.swap(lscan, rscan);
    }
    xs.swap(low, rscan);
    rscan
}

/// Implements the quicksort algorithm via Dijkstra's approach. The method is
/// based on a single left-to-right pass through the array. The goal is to
/// divide the array into three parts. Let `v` be an array and `i` be a pivot
/// index of `v`:
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
/// use algae::sort;
///
/// let mut v = "quicksortexample".as_bytes().to_vec();
/// sort::quick_three_way_sort(&mut v);
/// assert_eq!("aceeiklmopqrstux", &String::from_utf8(v).unwrap());
/// ```
pub fn quick_three_way_sort<T: PartialOrd + Copy>(xs: &mut [T]) {
    fastrand::shuffle(xs);
    quick_three_way_sort_rec(xs, 0, xs.len().saturating_sub(1));
}

fn quick_three_way_sort_rec<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) {
    if high <= low + CUTOFF {
        sort::insertion_sort(xs);
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
                gt = gt.saturating_sub(1);
            }
            Ordering::Equal => scan += 1,
        }
    }
    quick_three_way_sort_rec(xs, low, lt.saturating_sub(1));
    quick_three_way_sort_rec(xs, gt + 1, high);
}

#[cfg(test)]
mod test {
    use crate::sort;

    #[test]
    fn sorting_quicksort() {
        sort::check_sort_fn(super::quick_sort);
    }

    #[test]
    fn sorting_quick3waysort() {
        sort::check_sort_fn(super::quick_three_way_sort);
    }
}
