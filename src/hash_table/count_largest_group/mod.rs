// https://leetcode.com/problems/count-largest-group/description
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 1..=n {
            map.entry(Self::sumn(i))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        // println!("{:?}", map);
        // Constraints:
        //
        //     1 <= n <= 10^4
        let mut max = 0;
        let mut cnt = 0;
        for (_, v) in map {
            if v > max {
                max = v;
                cnt = 0;
            }
            if v == max {
                cnt += 1;
            }
        }
        cnt
    }

    #[inline]
    fn sumn(mut n: i32) -> i32 {
        // Constraints:
        //
        //     1 <= n <= 10^4
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_largest_group() {
        assert_eq!(2, Solution::count_largest_group(2));
        assert_eq!(9, Solution::count_largest_group(19));
        assert_eq!(5, Solution::count_largest_group(24));
        assert_eq!(9, Solution::count_largest_group(9));
        assert_eq!(4, Solution::count_largest_group(13));
    }
}
