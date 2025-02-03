struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut prev = arr[0];
        let mut i = 1;
        while i < arr.len() && arr[i] > prev {
            prev = arr[i];
            i += 1;
        }
        if i == arr.len() || i == 1 {
            return false;
        }
        while i < arr.len() && prev > arr[i] {
            prev = arr[i];
            i += 1;
        }
        i == arr.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_mountain_array() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
        assert!(
            !Solution::valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
        );
        assert!(
            Solution::valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0])
        );
        assert!(
            !Solution::valid_mountain_array(vec![0, 2, 3, 3, 5, 2, 1])
        );
    }
}
