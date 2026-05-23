// #953
// https://leetcode.com/problems/verifying-an-alien-dictionary/description/

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        // Declare the alphabet array. Initially all values will be == 255
        // which is higher than any English lowercase ASCII(97 - 122).
        // This is unnecessary since it is guaranteed that order.len() == 26 all lowercase English
        // letters, but still
        let mut alphabet: [usize; 26] = [255; 26];

        for (i, letter) in order.as_bytes().iter().enumerate() {
            // Set the alien letter order
            alphabet[(letter - 97) as usize] = i;
        }

        words.windows(2).all(|w| Self::le(&w[0], &w[1], &alphabet))
    }

    #[inline]
    fn le(a: &str, b: &str, alphabet: &[usize]) -> bool {
        let it = a.as_bytes().iter().zip(b.as_bytes().iter());

        for (a, b) in it {
            // Determine the alien lexicographical order for each of opposing letters
            let a_ord = alphabet[(a - 97) as usize];
            let b_ord = alphabet[(b - 97) as usize];

            // If a[i] == b[i] then proceed
            if a_ord == b_ord {
                continue;
            }

            // Return the inequality
            return a_ord < b_ord;
        }

        // all checked letters so far has equal order, so check that a is prefix of b
        a.len() <= b.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifying_alien_dict() {
        assert!(Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ));

        assert!(!Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ));

        assert!(!Solution::is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));

        assert!(Solution::is_alien_sorted(
            vec!["apple".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));

        assert!(Solution::is_alien_sorted(
            vec!["kuvp".to_string(), "q".to_string()],
            "ngxlkthsjuoqcpavbfdermiywz".to_string()
        ));
    }
}
