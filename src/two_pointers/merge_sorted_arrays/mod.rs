struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut left = 0;
        let mut right = 0;
        let (mut m, n) = (m as usize, n as usize);
        for i in 0..m + n {
            if right == n {
                break;
            }
            if left == m {
                nums1[i] = nums2[right];
                right += 1;
                continue;
            }
            if nums1[left] > nums2[right] {
                let mut num = nums2[right];
                for i in left..nums1.len() {
                    (nums1[i], num) = (num, nums1[i]);
                }
                left += 1;
                m += 1;
                right += 1;
            } else {
                nums1[i] = nums1[left];
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let mut arr = vec![1, 2, 3, 0, 0, 0];
        let mut arr2 = vec![2, 5, 6];
        Solution::merge(&mut arr, 3, &mut arr2, 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], arr);

        arr = vec![7, 8, 9, 0, 0, 0];
        arr2 = vec![2, 5, 6];
        Solution::merge(&mut arr, 3, &mut arr2, 3);
        assert_eq!(vec![2, 5, 6, 7, 8, 9], arr);

        arr = vec![1];
        arr2 = vec![];
        Solution::merge(&mut arr, 1, &mut arr2, 0);
        assert_eq!(vec![1], arr);
    }
}
