// #473
// https://leetcode.com/problems/matchsticks-to-square/description/
// https://www.youtube.com/watch?v=hUe0cUKV-YY

struct Solution;

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        fn backtrack(i: usize, side: i32, sides: &mut [i32; 4], matchsticks: &[i32]) -> bool {
            if i == matchsticks.len() {
                return true;
            }

            for s in 0..4 {
                if sides[s] + matchsticks[i] > side {
                    continue;
                }
                sides[s] += matchsticks[i];
                if backtrack(i + 1, side, sides, matchsticks) {
                    return true;
                }
                sides[s] -= matchsticks[i];
            }

            false
        }

        let perimeter = matchsticks.iter().sum::<i32>();
        if perimeter % 4 != 0 {
            return false;
        }

        matchsticks.sort_unstable_by(|a, b| b.cmp(a));

        let side = perimeter / 4;
        let mut sides = [0; 4];

        backtrack(0, side, &mut sides, &matchsticks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matchsticks_to_square() {
        assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
    }
}
