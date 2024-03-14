use std::collections::HashMap;

struct Solution {}

impl Solution {
    // recursion
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        Self::fib(n - 1) + Self::fib(n - 2)
    }

    // interative
    pub fn fib_iter(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut n_minus_one = 1;
        let mut n_minus_two = 0;
        for _ in 2..n {
            (n_minus_one, n_minus_two) = (n_minus_one + n_minus_two, n_minus_one);
        }
        n_minus_one + n_minus_two
    }

    // memoization
    pub fn fib_memo(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::fib_memo_helper(&mut memo, n)
    }

    pub fn fib_memo_helper(m: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if let Some(v) = m.get(&n) {
            return *v;
        }
        let res = if n < 2 {
            n
        } else {
            Self::fib_memo_helper(m, n - 1) + Self::fib_memo_helper(m, n - 2)
        };
        m.insert(n, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib() {
        assert_eq!(1, Solution::fib(2));
        assert_eq!(0, Solution::fib(0));
        assert_eq!(1, Solution::fib(1));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
        assert_eq!(5, Solution::fib(5));

        assert_eq!(1, Solution::fib_iter(2));
        assert_eq!(0, Solution::fib_iter(0));
        assert_eq!(1, Solution::fib_iter(1));
        assert_eq!(2, Solution::fib_iter(3));
        assert_eq!(3, Solution::fib_iter(4));
        assert_eq!(5, Solution::fib_iter(5));

        assert_eq!(1, Solution::fib_memo(2));
        assert_eq!(0, Solution::fib_memo(0));
        assert_eq!(1, Solution::fib_memo(1));
        assert_eq!(2, Solution::fib_memo(3));
        assert_eq!(3, Solution::fib_memo(4));
        assert_eq!(5, Solution::fib_memo(5));
    }
}
