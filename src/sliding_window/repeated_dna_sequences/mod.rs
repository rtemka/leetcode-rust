// #187
// https://leetcode.com/problems/repeated-dna-sequences/description/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut map: HashMap<&str, usize> = HashMap::new();

        for hi in 10..=s.len() {
            map.entry(&s[hi - 10..hi])
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        map.into_iter()
            .filter_map(
                |(s, count)| {
                    if count > 1 { Some(s.to_owned()) } else { None }
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_dna_sequences() {
        let mut expected = ["AAAAACCCCC", "CCCCCAAAAA"];
        let mut result =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());

        expected.sort_unstable();
        result.sort_unstable();

        assert_eq!(result, expected);

        let mut expected = ["AAAAAAAAAA"];
        let mut result = Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string());

        expected.sort_unstable();
        result.sort_unstable();

        assert_eq!(result, expected);
    }
}
