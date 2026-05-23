// https://leetcode.com/problems/longest-common-prefix/
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut bstrs: Vec<&[u8]> = Vec::with_capacity(strs.len());
        let mut shortest_idx = 0;
        for (i, str) in strs.iter().enumerate() {
            bstrs.push(str.as_bytes());
            if bstrs[i].len() < bstrs[shortest_idx].len() {
                shortest_idx = i;
            }
        }
        'outer: for i in (0..bstrs[shortest_idx].len()).rev() {
            let s = &bstrs[shortest_idx][0..=i];
            for &bstr in bstrs.iter() {
                if &bstr[0..=i] != s {
                    continue 'outer;
                }
            }
            return String::from_utf8(s.to_vec()).unwrap();
        }
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "".to_owned(),
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ])
        );
        assert_eq!(
            "hello",
            Solution::longest_common_prefix(vec![
                "hello".to_owned(),
                "hello".to_owned(),
                "hello".to_owned()
            ])
        );
        assert_eq!(
            "f",
            Solution::longest_common_prefix(vec!["f".to_owned(), "f".to_owned()])
        );
    }
}
