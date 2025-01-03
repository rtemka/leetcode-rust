use std::collections::{BinaryHeap, HashMap};

// https://leetcode.com/problems/top-k-frequent-elements/description/
struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for n in nums {
            map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        }
        // let mut heap = BinaryHeap::from_iter(map.into_iter().map(|(k, v)| (v, k)));
        // let mut res = Vec::with_capacity(k as usize);
        // for _ in 0..k {
        //     let (_, num) = unsafe { heap.pop().unwrap_unchecked() };
        //     res.push(num);
        // }
        // res

        // By Iterator
        BinaryHeap::from_iter(map.into_iter().map(|(k, v)| (v, k)))
            .into_sorted_vec()
            .into_iter()
            .rev()
            .take(k as usize)
            .map(|(_, n)| n)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_frequent() {
        assert_eq!(
            vec![1, 2],
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
        );
    }
}
