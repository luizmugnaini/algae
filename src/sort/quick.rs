use super::Sorter;
use rand::seq::SliceRandom;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        let mut rng = rand::thread_rng();
        xs.shuffle(&mut rng);
        QuickSort::_sort(xs, 0, xs.len().checked_sub(1).unwrap_or(0));
    }
}

impl QuickSort {
    fn _sort<T: PartialOrd + Copy>(xs: &mut [T], low: usize, high: usize) {
        if high <= low {
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
            while xs[lscan] < pivot {
                if lscan == high {
                    break;
                }
                lscan += 1;
            }

            while pivot < xs[rscan] {
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

#[cfg(test)]
mod test {
    use super::QuickSort;
    use crate::sort;

    #[test]
    fn sorting() {
        sort::check_sorter(QuickSort);
    }
}
