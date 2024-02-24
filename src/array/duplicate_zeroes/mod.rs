struct Solution {}

// https://leetcode.com/problems/duplicate-zeros/description/
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                let mut n = arr[i];
                for i in i + 1..arr.len() {
                    (arr[i], n) = (n, arr[i]);
                }
                i += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_zeroes() {
        let mut arr = vec![1, 2, 0, 3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(vec![1, 2, 0, 0], arr,);

        let mut arr = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(vec![1, 2, 3], arr,);

        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(vec![1, 0, 0, 2, 3, 0, 0, 4], arr,);
    }
}
