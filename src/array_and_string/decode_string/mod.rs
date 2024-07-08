// https://leetcode.com/problems/decode-string/description
struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        s
    }

    fn parse_string(s: &str) -> String {
        // let mut k = 1;
        // let mut k_str = "";
        let mut k_stack: Vec<usize> = Vec::new();
        let mut k_start = s.len();
        let mut str_stack: Vec<String> = Vec::new();
        let mut str_start = s.len();
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if str_start != s.len() {
                        str_stack.push(s[str_start..i].to_string());
                        str_start = s.len();
                    }
                    if k_start == s.len() {
                        k_start = i;
                    }
                }
                '[' => {
                    // k = s[k_start..i].parse::<usize>().unwrap();
                    k_stack.push(s[k_start..i].parse::<usize>().unwrap());
                    k_start = s.len();
                }
                ']' => {
                    let k = k_stack.pop().unwrap();
                    let mut new_str = String::with_capacity(s[str_start..i].len()*k);
                    for i in 0..k {
                        new_str.push_str(&s[str_start..i]);
                    }
                    // let mut str = str_stack.pop().unwrap(); 
                },
                _ => {
                    // cur_str.push(c)
                    if str_start == s.len() {
                        str_start = i;
                    }
                }
                ,
            }
        }
        res
        // for i in 0..k {
        //     res.push_str(&s[lo..hi]);
        // }
    }
        fn parse_string2(s: &str) -> String {
        let mut k = 1;
        let mut k_str = "";
        // let mut k_stack: Vec<usize> = Vec::new();
        let mut k_start = s.len();
        // let mut str_stack: Vec<String> = Vec::new();
        let mut str_start = s.len();
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' if k_start == s.len() => {
                        k_start = i;
                }
                '[' => {
                    k = s[k_start..i].parse::<usize>().unwrap();
                    // k_stack.push(s[k_start..i].parse::<usize>().unwrap());
                    // k_start = s.len();
                }
                ']' => break,
                _ => {
                    res.push(c);
                    // if str_start == s.len() {
                    //     str_start = i;
                    // }
                }
                ,
            }
        }
        res
        // for i in 0..k {
        //     res.push_str(&s[lo..hi]);
        // }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_string() {
        assert_eq!(
            "aaabcbc".to_string(),
            Solution::decode_string("3[a]2[bc]".to_string())
        );
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".to_string())
        );
        assert_eq!(
            "accbbacbb".to_string(),
            Solution::decode_string("2[a2[c2[b]]]".to_string())
        );
    }
}
