// #658
// https://leetcode.com/problems/find-k-closest-elements/description/

struct Solution;

impl Solution {
    pub fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut lo, mut hi, k) = match arr.binary_search(&x) {
            Ok(i) => (i as isize - 1, i + 1, k as usize - 1),
            Err(i) => (i as isize - 1, i, k as usize),
        };

        for i in 0..k {
            if lo < 0 {
                hi += k - i;
                break;
            }
            if hi == arr.len() {
                lo -= (k - i) as isize;
                break;
            }

            if x.abs_diff(arr[lo as usize]) <= x.abs_diff(arr[hi]) {
                lo -= 1;
            } else {
                hi += 1;
            }
        }

        arr.drain((lo + 1) as usize..hi).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_k_closest_elements() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
        );

        assert_eq!(
            vec![1, 1, 2, 3],
            Solution::find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, -1)
        );

        assert_eq!(vec![1], Solution::find_closest_elements(vec![1], 1, 1));

        assert_eq!(
            vec![10],
            Solution::find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9)
        );
    }
}
