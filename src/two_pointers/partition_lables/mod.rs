// #763
// https://leetcode.com/problems/partition-labels/description/
struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut result = Vec::new();
        // Store where each letter occurs for the last time.
        let mut last_index = [0usize; 26];
        for i in 0..s.len() {
            last_index[(s[i] - b'a') as usize] = i;
        }
        // Traverse the string again and keep track of the
        // farthest last occurrence you've seen so far.
        let (mut lo, mut hi) = (0, 0);
        for i in 0..s.len() {
            hi = usize::max(hi, last_index[(s[i] - b'a') as usize]);
            if i == hi {
                result.push((hi - lo + 1) as i32);
                lo = hi + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_lables() {
        assert_eq!(
            vec![9, 7, 8],
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
        )
    }
}
