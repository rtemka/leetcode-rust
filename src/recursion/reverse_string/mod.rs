struct Solution {}

// https://leetcode.com/problems/reverse-string/
impl Solution {
    #[allow(clippy::all)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut lo = 0;
        let mut hi = s.len() as i32 - 1;
        while lo < hi {
            (s[lo as usize], s[hi as usize]) = (s[hi as usize], s[lo as usize]);
            lo += 1;
            hi -= 1;
        }
    }

    pub fn reverse_string_rec(s: &mut [char]) {
        Self::swap_fst_last(s);
    }

    fn swap_fst_last(s: &mut [char]) {
        if s.len() < 2 {
            return;
        }
        let last = s.len() - 1;
        (s[0], s[last]) = (s[last], s[0]);
        Self::swap_fst_last(&mut s[1..last]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_string() {
        let str = String::from("something");
        let mut cv: Vec<char> = str.chars().collect();
        Solution::reverse_string(&mut cv);
        assert_eq!("gnihtemos".chars().collect::<Vec<_>>(), cv);

        let str = String::from("hello");
        let mut cv: Vec<char> = str.chars().collect();
        Solution::reverse_string(&mut cv);
        assert_eq!("olleh".chars().collect::<Vec<_>>(), cv);

        let str = String::from("Hannah");
        let mut cv: Vec<char> = str.chars().collect();
        Solution::reverse_string(&mut cv);
        assert_eq!("hannaH".chars().collect::<Vec<_>>(), cv);

        let str = String::from("hello");
        let mut cv: Vec<char> = str.chars().collect();
        Solution::reverse_string_rec(&mut cv);
        assert_eq!("olleh".chars().collect::<Vec<_>>(), cv);

        let mut vchar = ['h', 'e', 'l', 'l', 'o'].to_vec();
        Solution::reverse_string(&mut vchar);
        assert_eq!(['o', 'l', 'l', 'e', 'h'].to_vec(), vchar);
    }
}
