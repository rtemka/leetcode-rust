// https://leetcode.com/problems/find-the-highest-altitude/description
struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold((0, 0), |acc, x| (acc.0 + x, acc.1.max(acc.0 + x)))
            .1
        // // straightforward loop
        // let mut altitude = 0;
        // let mut cur = 0;
        // for i in gain {
        //     cur += i;
        //     altitude = altitude.max(cur);
        // }
        // altitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_altitudy() {
        assert_eq!(1, Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
        assert_eq!(0, Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }
}
