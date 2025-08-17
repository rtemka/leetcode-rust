pub mod maximum_subsequence_score;
pub mod top_k_frequent_words;
pub mod total_cost_to_hire_k_workers;

#[derive(Debug)]
pub struct Heap<T> {
    storage: Vec<T>,
}

impl<T> Heap<T>
where
    T: Ord + Default,
{
    /// Constructs a new, empty `Heap<T>`.
    pub fn new() -> Self {
        Self {
            storage: vec![T::default()],
        }
    }

    /// Constructs a new, empty `Heap<T>` with at least the specified capacity.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX - 1` _bytes_.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity + 1);
        v.push(T::default());
        Self { storage: v }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Heap<T> {
        Heap::from(iter.into_iter().collect::<Vec<_>>())
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Determines if heap `a` value has lower priority than `b` value.
    ///
    /// # Panics
    ///
    /// Panics if the current heap length is less then or equal `a` or `b`.
    #[inline(always)]
    fn less(&self, a: usize, b: usize) -> bool {
        assert!(self.storage.len() > a);
        assert!(self.storage.len() > b);
        self.storage[a] < self.storage[b]
    }

    /// Switches the locations of entries `a` and `b`.
    #[inline(always)]
    fn swap(&mut self, a: usize, b: usize) {
        assert!(self.storage.len() > a);
        assert!(self.storage.len() > b);
        self.storage.swap(a, b);
    }

    /// Place val in the next empty location and swim it upward.
    pub fn push(&mut self, val: T) {
        self.storage.push(val);
        self.swim(self.storage.len() - 1);
    }

    /// Remakes the storage array to conform to the heap-ordered property.
    #[inline(always)]
    fn swim(&mut self, mut child: usize) {
        while child > 1 && self.less(child / 2, child) {
            self.swap(child, child / 2);
            child /= 2;
        }
    }

    /// Pops val out of heap if any. The poped element is max/min according to heap property.
    pub fn pop(&mut self) -> Option<T> {
        if self.storage.len() < 2 {
            return None;
        }
        self.swap(1, self.storage.len() - 1);
        let max_entry = self.storage.pop();
        self.sink(1);
        max_entry
    }

    /// Reestablish the structure of the max binary heap after pop() is invoked.
    #[inline(always)]
    fn sink(&mut self, mut parent: usize) {
        while 2 * parent < self.storage.len() {
            let mut child = 2 * parent;
            if child < self.storage.len() - 1 && self.less(child, child + 1) {
                child += 1;
            }
            if !self.less(parent, child) {
                break;
            }
            self.swap(child, parent);
            parent = child;
        }
    }

    /// Rebuilds the heap.
    #[inline(always)]
    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 1 {
            n -= 1;
            self.sink(n);
        }
    }
}

impl<T: Ord + Default> From<Vec<T>> for Heap<T> {
    /// Converts a `Vec<T>` into a `Heap<T>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    fn from(vec: Vec<T>) -> Heap<T> {
        let mut heap = Heap { storage: vec };
        heap.storage.push(T::default());
        heap.swap(0, heap.len() - 1);
        heap.rebuild();
        heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_queue_len() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.storage.len(), 1);
        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.storage.len(), 4);
    }

    #[test]
    fn priority_queue_with_cap() {
        let heap = Heap::<usize>::with_capacity(5);
        assert_eq!(heap.storage.len(), 1);
        assert_eq!(heap.storage.capacity(), 6);
    }

    #[test]
    fn priority_queue_max_property() {
        let mut heap = Heap::<i32>::new();
        for i in 1..=10 {
            heap.push(i);
        }
        // println!("{:#?}", heap);
        for i in (1..=10).rev() {
            assert_eq!(heap.pop().unwrap(), i);
        }
        // println!("{:#?}", heap);
    }

    #[test]
    fn priority_queue_from_vec() {
        let v = vec![1, 2, 3, 4, 5];
        let mut heap = Heap::from(v.clone());
        // println!("{:#?}", heap);
        for n in v.into_iter().rev() {
            assert_eq!(n, heap.pop().unwrap());
        }
    }
    #[test]
    fn priority_queue_from_iter() {
        let v = vec![1, 2, 3, 4, 5];
        let mut heap = Heap::from_iter(v.clone());
        // println!("{:#?}", heap);
        for n in v.into_iter().rev() {
            assert_eq!(n, heap.pop().unwrap());
        }
    }
}
