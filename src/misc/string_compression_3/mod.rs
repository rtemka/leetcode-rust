// https://leetcode.com/problems/string-compression-iii/description
struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut comp = String::new();
        let mut cnt = 1;
        let mut last_char = word.chars().nth(0).unwrap();
        for c in word.chars().chain("0".chars()).skip(1) {
            if c != last_char || cnt == 9 {
                comp.push(DIGITS[cnt]);
                comp.push(last_char);
                cnt = 1;
                last_char = c;
            } else {
                cnt += 1;
            }
        }
        comp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compressed_string() {
        assert_eq!(
            "1a1b1c1d1e".to_owned(),
            Solution::compressed_string("abcde".to_owned())
        );
        assert_eq!(
            "9a5a2b".to_owned(),
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_owned())
        );
        assert_eq!("1a".to_owned(), Solution::compressed_string("a".to_owned()));
        assert_eq!(
            "5a9z4z".to_owned(),
            Solution::compressed_string("aaaaazzzzzzzzzzzzz".to_owned())
        );
    }
}
