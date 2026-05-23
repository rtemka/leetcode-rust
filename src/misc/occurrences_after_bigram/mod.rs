// https://leetcode.com/problems/occurrences-after-bigram/description/
struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut flag = false;
        let mut res = Vec::new();
        let mut prev = "";
        for w in text.split_ascii_whitespace() {
            if flag {
                res.push(w.to_owned());
            }
            flag = prev == first && w == second;
            prev = w;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occurrences_after_bigram() {
        assert_eq!(
            vec!["girl", "student"],
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string()
            )
        );

        assert_eq!(
            vec!["we", "rock"],
            Solution::find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string()
            )
        );

        assert_eq!(
            // Vec::<String>::new(),
            vec!["girl"],
            Solution::find_ocurrences(
                "a good girl".to_string(),
                "a".to_string(),
                "good".to_string()
            )
        );
    }
}
