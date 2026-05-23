// https://leetcode.com/problems/largest-number-at-least-twice-of-others/description/
struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut fst, mut snd) = (0, 1);
        for (i, _) in nums.iter().enumerate() {
            if nums[i] > nums[fst] {
                (fst, snd) = (i, fst);
            } else if i != fst && nums[i] > nums[snd] {
                snd = i;
            }
        }
        if nums[fst] / 2 < nums[snd] {
            -1
        } else {
            fst as i32
        }
    }
}
