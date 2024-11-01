// https://leetcode.com/problems/decode-string/description
struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let bs = s.as_bytes();
        let (v, _) = Self::parse_string2(bs);
        unsafe { String::from_utf8_unchecked(v) }
    }

    #[inline]
    fn parse_usize(bytes: &[u8]) -> usize {
        const RADIX: u32 = 10;
        let mut pow = 1;
        let mut res = 0;
        for &c in bytes.iter().rev() {
            res += (c as char).to_digit(RADIX).unwrap() * pow;
            pow *= 10;
        }
        res as usize
    }

    // 2[a2[c2[b]]]    // 3[a]2[bc]   // 3[a2[c]]
    fn parse_string2(bytes: &[u8]) -> (Vec<u8>, usize) {
        let mut v = Vec::new();
        let mut digit_start = bytes.len();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'0'..=b'9' => {
                    if digit_start == bytes.len() {
                        digit_start = i;
                    }
                }
                b'[' => {
                    let n = Self::parse_usize(&bytes[digit_start..i]);
                    digit_start = bytes.len();
                    let (vv, j) = Self::parse_string2(&bytes[i + 1..]);
                    i += j;
                    v.append(&mut vv.repeat(n));
                }
                b']' => break,
                _ => v.push(bytes[i]),
            }
            i += 1;
        }
        (v, i + 1)
    }

    //   // 2[a2[c2[b]]]
    // fn parse_string(s: &str) -> String {
    //     let mut digit_stack: Vec<usize> = Vec::new();
    //     let mut digit_start = s.len();
    //     let mut str_stack: Vec<String> = Vec::new();
    //     let mut str_start = s.len();
    //     let mut bracket_stack: Vec<char> = Vec::new();
    //     let mut res = String::new();
    //     for (i, c) in s.chars().enumerate() {
    //         match c {
    //             '0'..='9' => {
    //                 if str_start != s.len() {
    //                     str_stack.push(s[str_start..i].to_string());
    //                     str_start = s.len();
    //                 }
    //                 if digit_start == s.len() {
    //                     digit_start = i;
    //                 }
    //             }
    //             '[' => {
    //                 // k = s[k_start..i].parse::<usize>().unwrap();
    //                 bracket_stack.push(c);
    //                 digit_stack.push(s[digit_start..i].parse::<usize>().unwrap());
    //                 digit_start = s.len();
    //             }
    //             ']' => {
    //                 if str_start != s.len() {
    //                     str_stack.push(s[str_start..i].to_string());
    //                     str_start = s.len();
    //                 }
    //                 let d = digit_stack.pop().unwrap();
    //                 let s = str_stack.pop().unwrap();
    //                 // let mut new_str = String::with_capacity(s[str_start..i].len() * k);
    //                 let mut new_str = String::with_capacity(s.len() * d);
    //                 new_str.push_str(&s.repeat(d))
    //             }
    //             _ => {
    //                 if str_start == s.len() {
    //                     str_start = i;
    //                 }
    //             }
    //         }
    //     }
    //     res
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_string() {
        assert_eq!(
            "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
                .to_string(),
            Solution::decode_string("10[leetcode]".to_string())
        );
        assert_eq!(
            "aaabcbc".to_string(),
            Solution::decode_string("3[a]2[bc]".to_string())
        );
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".to_string())
        );
        // digit_buffer: 3,2
        // bracket_buff: [,[
        // char_buffer_: a,c
        //               accaccacc cc
        assert_eq!(
            "acbbcbbacbbcbb".to_string(),
            Solution::decode_string("2[a2[c2[b]]]".to_string())
        );
        // digit_buffer: 2,2,2
        // bracket_buff: [,[,[
        // char_buffer_: a,c,b
        //               acbbcbbacbbcbb  cbbcbb  bb
    }
}
