// #657
// https://leetcode.com/problems/robot-return-to-origin/description/

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut origin = (0, 0);

        for m in moves.chars() {
            match m {
                'U' => origin.1 += 1,
                'D' => origin.1 -= 1,
                'L' => origin.0 -= 1,
                'R' => origin.0 += 1,
                _ => unreachable!(),
            }
        }

        origin == (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robot_return_to_origin() {
        assert!(Solution::judge_circle("UD".to_string()));
        assert!(!Solution::judge_circle("LL".to_string()));
        assert!(Solution::judge_circle("UULLUDRRDD".to_string()));
    }
}
