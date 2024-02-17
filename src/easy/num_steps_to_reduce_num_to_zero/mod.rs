struct Solution {}

impl Solution {
    // digits - count digits in the n.
    fn digits(n: i32) -> i32 {
        let mut count = 0;
        let mut i = n;
        while i != 0 {
            i = i / 10;
            count += 1;
        }
        count
    }

    /// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    fn number_of_steps(n: i32) -> i32 {
        let mut count = 0;
        let mut i = n;
        while i != 0 {
            if i % 2 == 0 {
                i /= 2;
            } else {
                i -= 1;
            }
            count += 1;
        }
        count
    }

    /// reverse every word in a [&str].
    fn reverse_words(str: &str) -> String {
        let mut res = String::with_capacity(str.len());
        for s in str.split(' ') {
            for c in s.chars().rev() {
                if c == ' ' {
                    continue;
                }
                res.push(c)
            }
            res.push(' ');
        }
        res.truncate(res.len() - 1);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits() {
        assert_eq!(1, Solution::digits(1));
        assert_eq!(2, Solution::digits(10));
        assert_eq!(3, Solution::digits(100));
    }

    #[test]
    fn number_of_steps() {
        assert_eq!(6, Solution::number_of_steps(14));
        assert_eq!(4, Solution::number_of_steps(8));
    }

    #[test]
    fn reverse_words() {
        assert_eq!(
            "sihT si na !elpmaxe",
            Solution::reverse_words("This is an example!")
        );
        assert_eq!(
            "elbuod  decaps  sdrow",
            Solution::reverse_words("double  spaced  words")
        );
    }
}
