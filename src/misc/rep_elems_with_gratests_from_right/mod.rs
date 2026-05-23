struct Solution {}

// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/description/
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut max = -1;
        for i in (0..arr.len()).rev() {
            let cur = arr[i];
            arr[i] = max;
            if cur > max {
                max = cur;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_elements() {
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
        assert_eq!(
            Vec::<i32>::new(),
            Solution::replace_elements(Vec::<i32>::new())
        );
        assert_eq!(vec![-1], Solution::replace_elements(vec![400]));
    }
}
