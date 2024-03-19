use std::collections::HashMap;

struct Solution {}

// https://leetcode.com/problems/climbing-stairs/description/
impl Solution {
    // recursive
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 4 {
            return n;
        }
        Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
    }

    // memoization
    pub fn climb_stairs_memo(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::climb_memo_helper(&mut memo, n)
    }

    pub fn climb_memo_helper(m: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if let Some(v) = m.get(&n) {
            return *v;
        }
        let res = if n < 3 {
            n
        } else {
            Self::climb_memo_helper(m, n - 1) + Self::climb_memo_helper(m, n - 2)
        };
        m.insert(n, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climb_stairs() {
        assert_eq!(2, Solution::climb_stairs_memo(2));
        assert_eq!(3, Solution::climb_stairs_memo(3));
        assert_eq!(5, Solution::climb_stairs_memo(4));
        assert_eq!(8, Solution::climb_stairs_memo(5));
        assert_eq!(13, Solution::climb_stairs_memo(6));
    }
}

// 4
// 1 + 1 + 1 + 1
// 2 + 2
// 2 + 1 + 1
// 1 + 2 + 1
// 1 + 1 + 2
//
// 5
// 1 + 1 + 1 + 1 + 1
// 2 + 2 + 1
// 1 + 2 + 2
// 2 + 1 + 2
// 2 + 1 + 1 + 1
// 1 + 2 + 1 + 1
// 1 + 1 + 2 + 1
// 1 + 1 + 1 + 2
//
// 6
// 1 + 1 + 1 + 1 + 1 + 1
// 2 + 2 + 2
// 2 + 2 + 1 + 1
// 1 + 2 + 2 + 1
// 1 + 1 + 2 + 2
// 1 + 2 + 1 + 2
// 2 + 1 + 2 + 1
// 2 + 1 + 1 + 1 + 1
// 1 + 2 + 1 + 1 + 1
// 1 + 1 + 2 + 1 + 1
// 1 + 1 + 1 + 2 + 1
// 1 + 1 + 1 + 1 + 2
