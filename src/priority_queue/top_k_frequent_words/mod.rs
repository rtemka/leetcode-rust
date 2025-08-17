// #692
// https://leetcode.com/problems/top-k-frequent-words/description/
struct Solution;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pair<'a> {
    freq: usize,
    str: &'a str,
}

impl Ord for Pair<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq).then(other.str.cmp(self.str))
    }
}

impl PartialOrd for Pair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.str == other.str
    }
}

impl Eq for Pair<'_> {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        // Create a hash map that counts the frequency of each string.
        let map = {
            let mut map: HashMap<&str, Pair> = HashMap::new();
            for word in &words {
                map.entry(word)
                    .and_modify(|pair| pair.freq += 1)
                    .or_insert(Pair { freq: 1, str: word });
            }
            map
        };
        // We could use into_iter_sorted in the future versions.
        // https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html#method.into_iter_sorted
        // Heapify on all string - frequency pairs.
        let mut heap: BinaryHeap<Pair> = BinaryHeap::from_iter(map.into_values().into_iter());
        // Pop the most frequent string off the heap 'k' times and return
        // these 'k' most frequent strings.
        let mut v = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some(el) = heap.pop() {
                v.push(el.str.to_string());
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_k_frequent_words() {
        assert_eq!(
            vec!["i".to_string(), "love".to_string()],
            Solution::top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            )
        )
    }
}
