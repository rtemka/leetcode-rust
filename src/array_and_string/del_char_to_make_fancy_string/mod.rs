// https://leetcode.com/problems/delete-characters-to-make-fancy-string/description
struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut cnt = 0;
        let mut cur_char = '0';
        s.chars()
            .filter(|&c| {
                if c == cur_char {
                    cnt += 1;
                } else {
                    cnt = 1;
                }
                cur_char = c;
                cnt < 3
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_fancy_string() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::make_fancy_string("leeetcode".to_string())
        );
        assert_eq!(
            "aabaa".to_string(),
            Solution::make_fancy_string("aaabaaaa".to_string())
        );
        assert_eq!(
            "aab".to_string(),
            Solution::make_fancy_string("aab".to_string())
        );
    }
}
