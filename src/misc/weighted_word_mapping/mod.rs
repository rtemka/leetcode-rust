// #3838
// https://leetcode.com/problems/weighted-word-mapping/description

struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut result = String::with_capacity(words.len());

        for word in words {
            let weight = Self::word_weight(&word, &weights);
            let letter = (weight % 26 - 25).unsigned_abs() as u8;
            result.push((letter + b'a') as char);
        }

        result
    }

    #[inline]
    fn word_weight(word: &str, weights: &[i32]) -> i32 {
        word.as_bytes()
            .iter()
            .fold(0_i32, |acc, &c| acc + weights[(c - b'a') as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weighted_word_mapping() {
        assert_eq!(
            "rij",
            Solution::map_word_weights(
                vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()],
                vec![
                    5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7,
                    2
                ]
            )
        );
    }
}
