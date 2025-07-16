// #503
// https://leetcode.com/problems/next-greater-element-ii/description/
struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut stack = Vec::<i32>::new();
        // Find the next largest number of each element, starting with the rightmost element.
        // But this time as opposed to `next greater element 1` we need to process `nums` 2 times.
        for i in (0..nums.len() * 2).rev() {
            // Determine the index in the `nums` and in the `result`.
            let idx = i % nums.len();
            // Pop values from the top of the stack until the current
            // value's next largest number is at the top.
            // while let Some(_) = stack.pop_if(|x| *x <= nums[idx]) {}
            while let Some(_) = stack.last().filter(|&&x| x <= nums[idx]) {
                stack.pop();
            }
            // Record the current value's next largest number,
            // which is at the top of the stack. If the stack is empty, record -1.
            result[idx] = *stack.last().unwrap_or(&(-1));
            stack.push(nums[idx]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_greater_element_2() {
        assert_eq!(
            vec![2, 3, 4, -1, 4],
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3])
        );
    }
}
