/// A heap is a complete binary tree, that is, all levels are full except
/// possibly for the last one.
pub trait Heap<T> {
    /// The length is the number of elements in the array representing the heap,
    /// and here one has `length >= size`.
    fn length(&self) -> usize;

    /// The heap size is the number of elements in the heap stored within the
    /// representing array. Any element of index equal to or greater than
    /// `size` is *not* an element of the heap, even though such element
    /// might exist in the original array. Therefore `size <= length`.
    fn size(&self) -> usize;

    /// Given a node index, returns the index of its parent node.
    fn parent(&self, node_idx: usize) -> Option<usize>;

    /// Returns the index of the node at the left relative to the given node
    /// index.
    fn left(&self, node_idx: usize) -> Option<usize>;

    /// Returns the index of the node at the right relative to the given node
    /// index.
    fn right(&self, node_idx: usize) -> Option<usize>;

    /// The height of a node in a heap is the number of edges on the *longest
    /// simple downward path from the node to a leaf*. The height of the
    /// heap itself is defined to be the height of its root.
    fn height(&self) -> usize;

    /// Given an index, we check if the corresponding node contains a valid
    /// element of the heap, if negative, we return a `None` - otherwise we
    /// compute the height of the valid node wrapped in an `Option` type.
    fn node_height(&self, node_idx: usize) -> Option<usize>;

    /// Builds a heap out of a vector.
    fn from_vec(data: Vec<T>) -> Self;

    /// Maintainence of the heap property starting from a given node index using
    /// a top-down approach. Runs in O(log n).
    fn heapify_top(&mut self, start_node_idx: usize);

    /// Maintainence of the heap property starting from a given node index using
    /// a bottom-up approach.
    fn heapify_bottom(&mut self, start_node_idx: usize);

    /// Given a recently initialized heap, assert the heap-property for each
    /// subtree. This method should not be invoked by the user, but only
    /// when creating a heap. Runs in O(n).
    fn build(self) -> Self;

    /// Given an index, returns the value stored at that node.
    fn node_at(&self, node_idx: usize) -> T;

    /// Change the size of the heap with respect to the representing array,
    /// maintaning the heap property.
    fn change_size_to(&mut self, new_heap_size: usize) -> Result<(), &str>;

    /// Push a new node to the heap and maintain the heap property.
    fn push(&mut self, new_node: T);
}

// TODO: min-heap should also be implemented.

/// A max-heap is a heap data structure where the parent of each node holds a
/// value greater than or equal to its children. From this, we see that the root
/// node holds the biggest value of the heap.
#[derive(Debug)]
pub struct MaxHeap<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: PartialOrd + Clone> Heap<T> for MaxHeap<T> {
    fn length(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> usize {
        self.size
    }

    #[inline]
    fn parent(&self, node_idx: usize) -> Option<usize> {
        if node_idx != 0 && node_idx < self.size {
            Some((node_idx - 1) / 2)
        } else {
            None
        }
    }

    #[inline]
    fn left(&self, node_idx: usize) -> Option<usize> {
        let left = 2 * node_idx + 1;
        if left < self.size {
            Some(left)
        } else {
            None
        }
    }

    #[inline]
    fn right(&self, node_idx: usize) -> Option<usize> {
        let right = 2 * (node_idx + 1);
        if right < self.size {
            Some(right)
        } else {
            None
        }
    }

    #[inline]
    fn height(&self) -> usize {
        if self.size == 0 {
            0
        } else {
            self.size.ilog2() as usize
        }
    }

    fn node_height(&self, node_idx: usize) -> Option<usize> {
        // Verifies if the index pertains to a valid element of the heap.
        if node_idx > self.size {
            return None;
        } else {
            // Goes down left until reaching an invalid heap node.
            let mut i = node_idx.clone();
            let mut height = 0;
            loop {
                match self.left(i) {
                    Some(left) => {
                        height += 1;
                        i = left;
                    }
                    None => break,
                }
            }
            return Some(height);
        }
    }

    fn from_vec(data: Vec<T>) -> Self {
        let size = data.len();
        let heap = Self { data, size };
        heap.build()
    }

    fn heapify_top(&mut self, idx: usize) {
        // We assume that the left and right children of the node at `idx` are both
        // max-heaps (whenever the children exist), and try finding the index
        // (`largest`) of the of the children and swapping it with our given
        // node at `idx`. After this procedure, we should continue the search
        // recursively down the heap to allocate the node at the correct place,
        // the base case is when `largest` is `idx` itself, in which case the node is
        // already in its correct location.
        let mut largest = idx;

        if let Some(left) = self.left(idx) {
            if self.data[left] > self.data[idx] {
                largest = left;
            };
        }

        if let Some(right) = self.right(idx) {
            if self.data[right] > self.data[largest] {
                largest = right;
            }
        }

        if largest != idx {
            self.data.swap(largest, idx);
            self.heapify_top(largest);
        }
    }

    fn heapify_bottom(&mut self, start_node_idx: usize) {
        if let Some(parent) = self.parent(start_node_idx) {
            if self.data[parent] < self.data[start_node_idx] {
                self.data.swap(parent, start_node_idx);
                self.heapify_bottom(parent);
            }
        }
    }

    fn build(mut self) -> Self {
        for current_idx in (0..(self.size / 2)).rev() {
            self.heapify_top(current_idx);
        }
        return self;
    }

    #[inline]
    fn node_at(&self, node_idx: usize) -> T {
        self.data[node_idx].clone()
    }

    #[inline]
    fn change_size_to(&mut self, new_size: usize) -> Result<(), &str> {
        assert!(
            new_size <= self.data.len(),
            "Casting heap size to {} is invalid since the representing array has length {}.",
            new_size,
            self.data.len()
        );
        self.size = new_size;
        Ok(())
    }

    fn push(&mut self, new_node: T) {
        // When pushing a new element to the heap, we should make sure that the `size`
        // of the heap is increased so that we can start tracking the new node.
        // Since `size <= length` at any point in time, by increasing the `size`
        // by 1 we can *at most* have `size = length + 1` and therefore we are
        // allowed to use the insert method to the vector, since `size - 1 <= length`.
        self.data.insert(self.size, new_node);
        self.size += 1;
        self.heapify_bottom(self.size - 1);
    }
}

impl<T: PartialOrd + Clone> MaxHeap<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Heapsort algorithm. Runs in O(n * log n).
    pub fn heapsort(data: Vec<T>) -> Vec<T> {
        let mut h = MaxHeap::from_vec(data);
        for idx in (1..h.length()).rev() {
            h.data.swap(0, idx);
            h.size -= 1;
            h.heapify_top(0);
        }
        h.data
    }
}

// TODO: implement more tests.
#[cfg(test)]
mod test {
    use super::*;
    use crate::sort;

    #[test]
    fn build_heap_from_vec() {
        let heap = MaxHeap::from_vec(vec![9, 3, 1, 2, 4, 16, 10, 7, 8, 14]);
        assert_eq!(heap.into_vec(), vec![16, 14, 10, 8, 4, 1, 9, 7, 2, 3]);
    }

    #[test]
    fn build_heap_via_push() {
        let mut heap = MaxHeap::new();
        vec![9, 3, 1, 2, 4, 16, 10, 7, 8, 14]
            .iter()
            .for_each(|&x| heap.push(x));
        assert_eq!(heap.into_vec(), vec![16, 14, 10, 7, 8, 1, 9, 2, 4, 3]);
    }

    #[test]
    fn heapsort() {
        let v = sort::rand_vec(1000);
        let v_heapsort = MaxHeap::heapsort(v);
        assert!(sort::is_sorted(&v_heapsort));
    }
}
