use std::collections::HashSet;

// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/description
struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut set: HashSet<&str> = HashSet::new();
        for entry in &folder {
            set.insert(entry);
        }
        folder
            .iter()
            .cloned()
            .filter(|s| {
                for (i, c) in s.char_indices().skip(1) {
                    if c == '/' {
                        if set.contains(&s[..i]) {
                            return false;
                        }
                    }
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_subfolders() {
        assert_eq!(
            vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()],
            Solution::remove_subfolders(vec![
                "/a".to_string(),
                "/a/b".to_string(),
                "/c/d".to_string(),
                "/c/d/e".to_string(),
                "/c/f".to_string()
            ])
        );

        assert_eq!(
            vec!["/a".to_string()],
            Solution::remove_subfolders(vec![
                "/a".to_string(),
                "/a/b/c".to_string(),
                "/a/b/d".to_string()
            ])
        );
        assert_eq!(
            vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string()
            ],
            Solution::remove_subfolders(vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string()
            ])
        );
    }
}
