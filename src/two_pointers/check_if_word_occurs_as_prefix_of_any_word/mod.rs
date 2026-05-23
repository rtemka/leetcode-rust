// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description
struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words = sentence.split_whitespace();
        for (i, word) in words.enumerate() {
            if word.len() >= search_word.len() && word[0..search_word.len()] == search_word {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prefix_of_word() {
        assert_eq!(
            4,
            Solution::is_prefix_of_word(String::from("i love eating burger"), String::from("burg"))
        );
        assert_eq!(
            2,
            Solution::is_prefix_of_word(
                String::from("this problem is an easy problem"),
                String::from("pro")
            )
        );
        assert_eq!(
            -1,
            Solution::is_prefix_of_word(String::from("i am tired"), String::from("you"))
        );
    }
}
