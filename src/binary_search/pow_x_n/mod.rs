// https://leetcode.com/problems/powx-n/description/
struct Solution {}

impl Solution {
    pub fn my_pow_std(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
    pub fn my_pow_rec(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 1.0 {
            return x;
        }

        if n < 0 {
            1.0 / Self::my_pow_rec_helper(x, -n)
        } else {
            Self::my_pow_rec_helper(x, n)
        }
    }

    pub fn my_pow_rec_helper(x: f64, n: i32) -> f64 {
        if n == 1 {
            x
        } else {
            x * Self::my_pow_rec(x, n - 1) // tail call?
        }
    }

    pub fn my_pow_loop(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 1.0 {
            return x;
        }
        let (n, negative) = if n < 0 { (-n, true) } else { (n, false) };
        let mut res = x;
        println!("{},{}", n, negative);
        for _ in 1..n {
            println!("{}", res);
            res *= x;
        }
        if negative {
            1.0 / res
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_pow() {
        assert_eq!(1024.0000, Solution::my_pow_std(2.0000, 10));
        assert_eq!(1024.0000, Solution::my_pow_rec(2.0000, 10));
        // assert_eq!(9.2610, Solution::my_pow_rec(2.1000, 3));
        assert_eq!(0.2500, Solution::my_pow_rec(2.0000, -2));
        assert_eq!(1024.0000, Solution::my_pow_loop(2.0000, 10));
        assert_eq!(1.0000, Solution::my_pow_loop(1.0000, 2147483647));
    }
}
