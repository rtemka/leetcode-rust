use std::collections::HashMap;

// https://leetcode.com/problems/unique-binary-search-trees/description/
struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut m = HashMap::with_capacity(n as usize);
        Self::num_trees_memo(&mut m, n)
    }

    pub fn num_trees_memo(m: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if let Some(v) = m.get(&n) {
            return *v;
        }
        let res = if n == 0 {
            1
        } else if n < 3 {
            n
        } else {
            let mut r = 0;
            for i in 0..n {
                r += Self::num_trees_memo(m, i) * Self::num_trees_memo(m, n - 1 - i);
            }
            r
        };
        m.insert(n, res);
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num_trees() {
        assert_eq!(5, Solution::num_trees(3));
        assert_eq!(1, Solution::num_trees(1));
        assert_eq!(14, Solution::num_trees(4));
    }
}
