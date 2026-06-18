// #2833
// https://leetcode.com/problems/furthest-point-from-origin/description/

struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut l, mut r) = (0, 0);

        for m in moves.chars() {
            match m {
                'R' => {
                    r += 1;
                    l -= 1;
                }
                'L' => {
                    l += 1;
                    r -= 1;
                }
                '_' => {
                    l += 1;
                    r += 1;
                }
                _ => unreachable!(),
            }
        }

        i32::max(l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn furthest_point_from_origin() {
        assert_eq!(
            3,
            Solution::furthest_distance_from_origin("L_RL__R".to_string())
        );

        assert_eq!(
            5,
            Solution::furthest_distance_from_origin("_R__LL_".to_string())
        );

        assert_eq!(
            7,
            Solution::furthest_distance_from_origin("_______".to_string())
        );
    }
}
