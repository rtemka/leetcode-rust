// https://leetcode.com/problems/k-th-symbol-in-grammar/description/
struct Solution {}

impl Solution {
    // This solution would cause TOO LONG TO RUN error.
    pub fn kth_grammar_rec_tooooo_long(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let v = Vec::with_capacity(k as usize);
        let res = Self::kth_grammar_helper(n, k, v);
        res[k as usize - 1]
    }

    // right recursive solution
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if k % 2 == 0 {
            Self::kth_grammar(n - 1, k / 2)
        } else {
            Self::kth_grammar(n - 1, k / 2 + 1)
        }
    }
    pub fn kth_grammar_helper(n: i32, k: i32, mut v: Vec<i32>) -> Vec<i32> {
        if k < v.len() as i32 {
            return v;
        }
        if n == 2 {
            v.push(0);
            v.push(1);
            return v;
        }
        let mut v = Self::kth_grammar_helper(n - 1, k, v);
        for i in v.len() / 2..v.len() {
            if v[i] == 0 {
                v.push(0);
                v.push(1);
            } else {
                v.push(1);
                v.push(0);
            }
        }
        println!("{:?}", v);
        v
    }
}

// 1 => 0
// 2 => 01
// 3 => 0110
// 4 => 01101001
// 5 => 0110100110010110
// 6 => 01101001100101101001011001101001

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_grammar() {
        assert_eq!(0, Solution::kth_grammar_rec_tooooo_long(1, 1));
        // assert_eq!(99, Solution::kth_grammar_rec(5, 1));
        assert_eq!(1, Solution::kth_grammar_rec_tooooo_long(3, 3));
        assert_eq!(0, Solution::kth_grammar_rec_tooooo_long(5, 16));
        assert_eq!(0, Solution::kth_grammar(30, 434991989));
    }
}
