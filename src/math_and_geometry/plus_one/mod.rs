// https://leetcode.com/problems/plus-one/description/
struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = false;
        let len = digits.len();
        for (i, n) in digits.iter_mut().enumerate().rev() {
            match *n {
                9 if i == 0 && carry || len == 1 => (*n, carry) = (1, true),
                9 => (*n, carry) = (0, true),
                _ => {
                    (*n, carry) = (*n + 1, false);
                    break;
                }
            }
        }
        if carry {
            digits.reserve_exact(1);
            digits.push(0);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one() {
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
        assert_eq!(vec![8], Solution::plus_one(vec![7]));
        assert_eq!(vec![1, 0, 0], Solution::plus_one(vec![9, 9]));
        assert_eq!(vec![1, 0, 1], Solution::plus_one(vec![1, 0, 0]));
        assert_eq!(vec![1, 7, 0], Solution::plus_one(vec![1, 6, 9]));
        assert_eq!(vec![3, 0, 0], Solution::plus_one(vec![2, 9, 9]));
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }
}
