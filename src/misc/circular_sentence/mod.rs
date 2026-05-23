// https://leetcode.com/problems/circular-sentence/description
struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.as_bytes();
        if sentence[0] != sentence[sentence.len() - 1] {
            return false;
        }
        let mut last_char = b'0';
        let mut space = false;
        for &c in sentence {
            if space && c != last_char {
                return false;
            }
            space = c == b' ';
            if !space {
                last_char = c;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_circular_sentence() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ))
    }
}
