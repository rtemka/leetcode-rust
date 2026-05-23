// https://leetcode.com/problems/find-the-difference/description
struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        println!("\n");
        let mut letters: [isize; 0x7B - 0x61] = [0; 0x7B - 0x61];
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        for i in 0..sb.len() {
            letters[sb[i] as usize - 0x61] += 1;
            letters[tb[i] as usize - 0x61] -= 1;
        }
        letters[tb[tb.len() - 1] as usize - 0x61] -= 1;
        for (i, &count) in letters.iter().enumerate() {
            if count < 0 {
                println!("{:?}", letters);
                return (0x61 + i as u8) as char;
            }
        }
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_the_diff() {
        assert_eq!(
            'e',
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
        );
        assert_eq!(
            'a',
            Solution::find_the_difference(
                "zcdoqwoekkdnccsdfeierwerwqewqeoowoeewkjgdkjsdfskjdsfn".to_string(),
                "zacdoqwoekkdnccsdfeierwerwqewqeoowoeewkjgdkjsdfskjdsfn".to_string()
            )
        );
        assert_eq!(
            'z',
            Solution::find_the_difference(
                "abdffbxcvxcvd".to_string(),
                "abdffbxczvxcvd".to_string()
            )
        );
    }
}
