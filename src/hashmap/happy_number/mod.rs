use std::collections::HashSet;

// https://leetcode.com/problems/happy-number/description/
struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        let mut rem;
        let mut square_sum;
        loop {
            square_sum = 0;
            while n > 0 {
                rem = n % 10;
                square_sum += rem * rem;
                n /= 10;
            }
            if square_sum == 1 {
                return true;
            }
            if !set.insert(square_sum) {
                break;
            }
            n = square_sum;
        }
        false
    }

    // solution with cycle detection
    // where fast and slow pointers is used.
    pub fn is_happy_fast_slow_pointers(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Solution::nxt(n);
        while fast != 1 && slow != fast {
            slow = Solution::nxt(slow);
            fast = Solution::nxt(Solution::nxt(fast));
        }
        fast == 1
    }

    fn nxt(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            res += d * d;
            n /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_happy() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(3));
        assert!(!Solution::is_happy(2));
        assert!(!Solution::is_happy(46));
    }
}
