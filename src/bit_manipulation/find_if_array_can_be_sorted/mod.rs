// https://leetcode.com/problems/find-if-array-can-be-sorted/description
struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut i = 1;
        let mut prev_group_max = i32::MIN;
        let mut cur_group_max = nums[0];
        let mut group_ones = cur_group_max.count_ones();
        // Слева направо, самый большой элемент предыдущего
        // сегмента должен быть меньше, чем самый маленький
        // элемент текущего сегмента.
        while i < nums.len() {
            if nums[i].count_ones() != group_ones {
                prev_group_max = cur_group_max;
                cur_group_max = nums[i];
                group_ones = nums[i].count_ones();
                continue;
            } else {
                if nums[i] < prev_group_max {
                    return false;
                }
                cur_group_max = cur_group_max.max(nums[i]);
                i += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sort_array() {
        assert!(Solution::can_sort_array(vec![
            1, 32, 2, 8, 2, 64, 4, 8, 16, 64, 64, 2, 64, 256, 2, 2, 2, 256, 64, 64, 1, 2, 256, 2,
            16, 256, 256, 2, 1, 256, 64, 8, 64, 16, 4, 32, 256, 16, 128, 8, 64, 64, 32, 4, 2, 16,
            8, 32, 8, 32, 128, 2, 16, 4, 16, 32, 16, 2, 64, 128, 8, 64, 8, 64, 64, 16, 256, 64, 1,
            16, 256, 1, 64, 256, 128, 2, 4, 128, 64, 256, 16, 4, 1, 8, 32, 32, 8, 1, 128, 1, 128,
            64, 32, 8, 64, 256, 128, 8, 256, 32
        ]));
        assert!(!Solution::can_sort_array(vec![20, 6, 7, 10, 20, 6, 20]));
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
        assert!(!Solution::can_sort_array(vec![7, 13, 18, 17, 8, 16, 2]));
        assert!(!Solution::can_sort_array(vec![15, 128, 1, 255]));
        assert!(Solution::can_sort_array(vec![33, 134, 56, 234]));
    }
}
