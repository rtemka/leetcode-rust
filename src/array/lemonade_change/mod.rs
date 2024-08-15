// https://leetcode.com/problems/lemonade-change/description
struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        for b in bills {
            match b {
                5 => five += 1,
                10 => {
                    if five == 0 {
                        return false;
                    }
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    if (ten == 0 && five < 3) || (ten > 0 && five == 0) {
                        return false;
                    }
                    if ten > 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        five -= 3;
                    }
                }
                _ => (),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lemonade_change() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
        assert!(Solution::lemonade_change(vec![5, 10, 5, 20, 5, 10]));
    }
}
