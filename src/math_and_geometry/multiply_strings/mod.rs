// https://leetcode.com/problems/multiply-strings/description/
struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" {
            return num1;
        } else if num2 == "0" {
            return num2;
        }
        const OFFSET: u8 = 0x30;

        let (num1, num2) = if num1.len() > num2.len() {
            (num1.as_bytes(), num2.as_bytes())
        } else {
            (num2.as_bytes(), num1.as_bytes())
        };
        let mut res = Vec::<u8>::with_capacity(num1.len() + num2.len());

        for (i, &x) in num2.iter().rev().enumerate() {
            let mut carry = 0;
            for (j, &y) in num1.iter().rev().enumerate() {
                if x == 0 || y == 0 {
                    continue;
                }
                let sum = (x - OFFSET) * (y - OFFSET) + carry;
                // println!("x={};y={};sum={}", x as char, y as char, sum);
                if i + j >= res.len() {
                    res.push(sum % 10);
                    carry = sum / 10;
                } else {
                    let sum = sum + res[i + j];
                    res[i + j] = sum % 10;
                    carry = sum / 10;
                }
            }
            if carry > 0 {
                res.push(carry);
            }
        }
        // res.reverse();
        res.into_iter()
            .rev()
            .map(|x| (x + OFFSET) as char)
            .collect()
        // unsafe { String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_strings() {
        assert_eq!(
            "56088".to_string(),
            Solution::multiply(String::from("123"), String::from("456"))
        );
        assert_eq!(
            "999989900001".to_string(),
            Solution::multiply(String::from("99999"), String::from("9999999"))
        );
    }
}

//      1 2 3
//      4 5 6
//      7 3 8
//    6 1 5 k
//  8 6 1
//
//      9 8 8
