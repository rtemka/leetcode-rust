// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/
struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() {
            let n = nums[i] as usize - 1;
            if nums[i] == i as i32 + 1 || nums[n] == nums[i] {
                i += 1;
            } else {
                nums.swap(i, n);
            }
            // println!("{:?}", nums);
        }
        nums.iter()
            .enumerate()
            .filter_map(|(i, &x)| {
                if x != i as i32 + 1 {
                    Some(i as i32 + 1)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_disappeared_numbers() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
