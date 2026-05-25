// #457
// https://leetcode.com/problems/circular-array-loop/description/

struct Solution;

impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        // Iterate through each index and check the cycle using slow and fast pointers
        for i in 0..nums.len() {
            // Already visited/processed
            if nums[i] == 0 {
                continue;
            }

            // Capture the initial direction (positive or negative)
            let direction = nums[i];
            // Init fast and slow pointers
            let (mut slow, mut fast) = (i, Self::next(i, &nums));

            // If direction changes (product is negative) or becomes 0, path is invalid
            while nums[slow] * nums[fast] > 0 && nums[Self::next(fast, &nums)] * direction > 0 {
                if slow == fast {
                    // Is cycle is detected check that this is not self loop e.g [1]
                    if slow == Self::next(slow, &nums) {
                        break;
                    }
                    // Cycle detected
                    return true;
                }

                // Advance slow pointer by 1 step
                slow = Self::next(slow, &nums);
                // Advance fast pointer by 2 steps
                fast = Self::next(Self::next(fast, &nums), &nums);
            }

            // Mark the path as 0 (visited/invalid)
            // We only need to mark nodes that follow the same direction
            let mut marker = i;
            while nums[marker] * direction > 0 {
                // Mark and advance marker pointer
                (nums[marker], marker) = (0, Self::next(marker, &nums));
            }
        }

        false
    }

    #[inline]
    fn next(index: usize, nums: &[i32]) -> usize {
        let n = nums.len() as i32;
        (((index as i32 + nums[index]) % n + n) % n) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_array_loop() {
        assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));

        assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5, 6]));

        assert!(Solution::circular_array_loop(vec![1, -1, 5, 1, 4]));

        assert!(!Solution::circular_array_loop(vec![-1]));
    }
}
