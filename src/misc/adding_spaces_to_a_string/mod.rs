// https://leetcode.com/problems/adding-spaces-to-a-string/description
struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::with_capacity(s.len() + spaces.len());
        let mut prev = 0;
        for idx in spaces {
            result.push_str(&s[prev as usize..idx as usize]);
            result.push(' ');
            prev = idx;
        }
        result.push_str(&s[prev as usize..s.len()]); // last chunk
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_spaces() {
        assert_eq!(
            String::from("Leetcode Helps Me Learn"),
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
        );

        assert_eq!(
            String::from("i code in py thon"),
            Solution::add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9])
        );

        assert_eq!(
            String::from(" i"),
            Solution::add_spaces("i".to_string(), vec![0])
        );

        assert_eq!(
            String::from(" s p a c i n g"),
            Solution::add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6])
        );
    }
}
