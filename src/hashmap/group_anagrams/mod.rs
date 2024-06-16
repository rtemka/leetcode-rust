// https://leetcode.com/problems/group-anagrams/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());
        for s in strs {
            let cloned = s.clone();
            let mut sorted_v = cloned.chars().collect::<Vec<char>>();
            sorted_v.sort_unstable();
            let sorted: String = sorted_v.into_iter().collect();

            match map.get_mut(&sorted) {
                Some(v) => v.push(cloned),
                _ => {
                    map.insert(sorted, vec![cloned]);
                }
            }
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_anagrams() {
        let mut expect = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        for v in expect.iter_mut() {
            v.sort_unstable();
        }
        expect.sort_unstable();
        let mut ans = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        for v in ans.iter_mut() {
            v.sort_unstable();
        }
        ans.sort_unstable();

        assert_eq!(expect, ans);

        assert_eq!(
            vec![vec![""]],
            Solution::group_anagrams(vec!["".to_string()])
        );
        assert_eq!(
            vec![vec!["a"]],
            Solution::group_anagrams(vec!["a".to_string()])
        );
    }
}
