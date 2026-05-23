// https://leetcode.com/problems/zigzag-conversion/description/
struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let s = s.as_bytes();
        let mut res: Vec<u8> = Vec::with_capacity(s.len());
        for i in 0..num_rows {
            let mut j = i;
            let mut down = i != num_rows - 1;
            while j < s.len() {
                res.push(s[j]);
                j += if down { (num_rows - 1 - i) * 2 } else { i * 2 };
                down = match i {
                    0 => true,
                    _ if i == num_rows - 1 => false,
                    _ => !down,
                }
            }
        }
        unsafe { String::from_utf8_unchecked(res) }
    }
}

// P   A   H   N
// A P L S I I G
// Y   I   R
//
//
// P     I    N
// A   L S  I G
// Y A   H R
// P     I
//
// P     H
// A   S  I
// Y  I    R
// P L      I  G
// A         N

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zigzag_convert() {
        assert_eq!(
            "PHASIYIRPLIGAN".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 5)
        );
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
        assert_eq!(
            "PAYPALISHIRING".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 1)
        );
    }
}
