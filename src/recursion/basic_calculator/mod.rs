// https://leetcode.com/problems/basic-calculator/description/
struct Solution;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
}

impl Solution {
    const OFFSET: u8 = 0x30;

    pub fn calculate(s: String) -> i32 {
        let (result, _) = Self::calculate_rec(s.as_bytes());
        result
    }

    fn calculate_rec(s: &[u8]) -> (i32, usize) {
        let mut result = 0;
        let mut operator = Operator::Add;
        let mut i = 0;
        while i < s.len() {
            match s[i] {
                b'+' => operator = Operator::Add,
                b'-' => operator = Operator::Sub,
                c if c.is_ascii_digit() => {
                    let (d, end) = Self::parse_digit(&s[i..]);
                    // let ps = format!(
                    //     "{:?};d={};i={};op={:?}",
                    //     String::from_utf8_lossy(s),
                    //     d,
                    //     i,
                    //     operator
                    // );
                    // dbg!(ps);
                    result = Self::eval(operator, result, d);
                    i += end;
                }
                b'(' => {
                    let (d, end) = Self::calculate_rec(&s[i + 1..]);
                    // let ps = format!(
                    //     "{:?};d={};i={};op={:?}",
                    //     String::from_utf8_lossy(s),
                    //     d,
                    //     i,
                    //     operator
                    // );
                    // dbg!(ps);
                    result = Self::eval(operator, result, d);
                    i += end;
                }
                b')' => return (result, i + 1),
                _ => (),
            }
            i += 1;
        }
        (result, i)
    }

    #[inline(always)]
    fn parse_digit(s: &[u8]) -> (i32, usize) {
        let (mut res, mut i) = (0, 0);
        for &c in s {
            match c {
                c if c.is_ascii_digit() => res = res * 10 + (c - Self::OFFSET) as i32,
                _ => return (res, i - 1),
            }
            i += 1;
        }
        (res, i)
    }

    #[inline(always)]
    fn eval(operator: Operator, left: i32, right: i32) -> i32 {
        match operator {
            Operator::Add => left + right,
            Operator::Sub => left - right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_calculator() {
        assert_eq!(
            1 - (1 - (1 - (1 - (-1) - 1) - 1) - 1),
            Solution::calculate("1-(1-(1-(1-(-1)-1)-1)-1)".to_string())
        );

        assert_eq!(-(1 - (-2)), Solution::calculate("-( 1-( -2)) ".to_string()));

        assert_eq!(1 + 1, Solution::calculate("1 + 1".to_string()));

        assert_eq!(
            (1 + (4 + 5 + 2) - 3) + (6 + 8),
            Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string())
        );

        assert_eq!(
            -(1 + 2 + 3 + 4),
            Solution::calculate("-(1+2+3+4)".to_string())
        );

        assert_eq!(
            (-(1 + 2)) + (-(5 + 5)),
            Solution::calculate("(-(1+2))+(-(5+5))".to_string())
        );

        assert_eq!(
            (1 - (4 + 5 - 20) + 3) - (-2 + 6 + 8),
            Solution::calculate("(1-(4+5-20)+3)-(-2+6+8)".to_string())
        );
    }
}
