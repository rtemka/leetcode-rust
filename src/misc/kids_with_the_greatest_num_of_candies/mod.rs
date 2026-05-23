// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/description
struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let &max = unsafe { candies.iter().max().unwrap_unchecked() };
        candies
            .into_iter()
            .map(|c| c + extra_candies >= max)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kids_with_candiesp() {
        assert_eq!(
            vec![true, true, true, false, true],
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3)
        );
        assert_eq!(
            vec![true, false, false, false, false],
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1)
        );
        assert_eq!(
            vec![true, false, true],
            Solution::kids_with_candies(vec![12, 1, 12], 10)
        );
    }
}
