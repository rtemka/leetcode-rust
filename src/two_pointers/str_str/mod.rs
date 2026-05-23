struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h = haystack.as_bytes();
        let n = needle.as_bytes();
        let mut i = 0;
        while i + n.len() - 1 < h.len() {
            println!("haystack:{:?},needle{:?}", String::from_utf8(h[i..n.len()+i].to_vec()).unwrap(), String::from_utf8(n.to_vec()).unwrap());
            if &h[i..n.len()+i] == n {
                return i as i32;
            }
            i += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_str() {
        assert_eq!(
            0,
            Solution::str_str("sadbutsad".to_owned(), "sad".to_owned())
        );
        assert_eq!(
            -1,
            Solution::str_str("leetcode".to_owned(), "leeto".to_owned())
        );
        assert_eq!(
            3,
            Solution::str_str("leetcode".to_owned(), "tc".to_owned())
        );
        assert_eq!(
            0,
            Solution::str_str("l".to_owned(), "l".to_owned())
        );
        assert_eq!(
            15,
            Solution::str_str("leeeeeeeeeeeeeeet".to_owned(), "et".to_owned())
        );
    }
}
