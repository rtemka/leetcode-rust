// https://leetcode.com/problems/find-all-duplicates-in-an-array/description/
struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        // All the integers of nums are in the range [1, n]
        // so we change integer positions to -1 for the first time
        // and to 0 when and if we see it again.
        // Then collect all indexes where value == 0.
        // There is a solution where we get result in one pass thru array.
        // When we meet the value for the first time we flip it to negative
        // and when we see negative value we add it to result. But anyhow I solved it my way.
        let mut i = 0;
        while i < nums.len() {
            // println!("{i}\t{:?}", nums);
            let num = nums[i];
            if num < 1 {
                i += 1;
            } else if nums[num as usize - 1] < 1 {
                nums[num as usize - 1] = 0;
                i += 1;
            } else {
                nums[i] = -1;
                nums.swap(i, num as usize - 1);
            }
        }
        // println!("{:?}", nums);
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| (x == 0).then_some(i as i32 + 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_duplicates_in_array() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));

        assert_eq!(
            vec![2, 3,],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            Solution::find_duplicates(vec![1, 2, 1, 2, 3, 4, 3, 4, 5, 6, 5, 6])
        );
    }
}
