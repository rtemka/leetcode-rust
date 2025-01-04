use std::{collections::HashSet, usize};

// https://leetcode.com/problems/permutation-sequence/description/
struct Solution;

impl Solution {
    // ASCII offset of numbers
    const OFFSET: u8 = 0x30;

    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut s = String::with_capacity(n as usize);
        let mut set: HashSet<u8> = HashSet::with_capacity(n as usize);
        let mut v: Vec<String> = Vec::with_capacity(k as usize);

        Self::backtrack(&mut v, &mut s, &mut set, n as usize, k as usize);
        dbg!(&v);

        unsafe { v.pop().unwrap_unchecked() }
    }

    #[inline]
    fn backtrack(v: &mut Vec<String>, s: &mut String, set: &mut HashSet<u8>, n: usize, k: usize) {
        if s.len() == n {
            v.push(s.clone());
            return;
        }
        for i in 1..=n as u8 {
            if v.len() == v.capacity() {
                return;
            }
            if !set.insert(i) {
                continue;
            }
            s.push((i + Self::OFFSET) as char);
            Self::backtrack(v, s, set, n, k);
            s.pop();
            set.remove(&i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_sequence() {
        assert_eq!(String::from("213"), Solution::get_permutation(3, 3));
        assert_eq!(String::from("2314"), Solution::get_permutation(4, 9));
        assert_eq!(String::from("123"), Solution::get_permutation(3, 1));
    }
}
