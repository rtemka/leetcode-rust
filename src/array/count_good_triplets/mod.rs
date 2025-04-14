// https://leetcode.com/problems/count-good-triplets/description/
struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                for k in j + 1..arr.len() {
                    count += ((arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c) as i32;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_good_triplets() {
        assert_eq!(
            4,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
        );

        assert_eq!(
            0,
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1)
        );
    }
}
