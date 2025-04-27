// https://leetcode.com/problems/sqrtx/description/
struct Solution;

impl Solution {
    pub fn my_sqrt_lib(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }

    // Calculator formula approach.
    pub fn my_sqrt_formula(x: i32) -> i32 {
        let result = ((x as f64).ln() * 0.5).exp() as usize;
        if (result + 1) * (result + 1) <= x as usize {
            result as i32 + 1
        } else {
            result as i32
        }
    }

    // Binary search approach.
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as usize;
        let (mut lo, mut hi) = (0, x as usize);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if mid * mid == x {
                return mid as i32;
            } else if mid * mid > x {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        hi as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_x() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt_formula(8));
        assert_eq!(3, Solution::my_sqrt_formula(9));
        assert_eq!(3, Solution::my_sqrt(9));
    }
}
