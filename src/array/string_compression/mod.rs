// https://leetcode.com/problems/string-compression/description
struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        //   Constraints:
        //   1 <= chars.length <= 2000
        let mut char = chars[0];
        let mut lo = 0;
        let mut hi = 1;
        while hi < chars.len() {
            if chars[hi] != char {
                println!("hi={};lo={};hi-lo={}", hi, lo, hi - lo);
                (lo, hi) = Self::count_and_compress(lo, hi, chars);
                char = chars[hi];
            }
            println!("{:?}", chars);
            hi += 1;
        }
        Self::count_and_compress(lo, hi, chars);
        chars.len() as i32
    }

    #[inline]
    fn count_and_compress(mut lo: usize, mut hi: usize, chars: &mut Vec<char>) -> (usize, usize) {
        let diff = hi - lo;
        if diff > 1 {
            lo += 1;
            for n in diff.to_string().chars() {
                chars[lo] = n;
                lo += 1;
            }
            chars.drain(lo..hi);
            hi = lo;
        }
        (hi, hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress() {
        // Arrange
        let mut v = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        // Act
        let res = Solution::compress(&mut v);
        println!("{:?}", v);
        // Assert
        assert_eq!(6, res);
        assert_eq!(vec!['a', '2', 'b', '2', 'c', '3'], v);

        // 2
        // Arrange
        let mut v = vec!['a'];
        // Act
        let res = Solution::compress(&mut v);
        // Assert
        assert_eq!(1, res);
        assert_eq!(vec!['a'], v);

        // 3
        // Arrange
        let mut v = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        // Act
        let res = Solution::compress(&mut v);
        // Assert
        assert_eq!(4, res);
        assert_eq!(vec!['a', 'b', '1', '2'], v);
    }
}
