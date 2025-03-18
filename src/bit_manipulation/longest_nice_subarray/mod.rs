// https://leetcode.com/problems/longest-nice-subarray/description
struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, 1);
        let mut max_count = 1;
        let mut sum = nums[0];
        while hi < nums.len() {
            max_count = max_count.max(hi - lo);
            if lo == hi {
                sum = nums[hi];
                hi += 1;
                continue;
            }
            if nums[hi] & sum == 0 {
                sum |= nums[hi];
                hi += 1;
            } else {
                sum ^= nums[lo];
                lo += 1;
            }
        }
        max_count = max_count.max(hi - lo);
        max_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_nice_subarray() {
        assert_eq!(
            3,
            Solution::longest_nice_subarray(vec![135745088, 609245787, 16, 2048, 2097152])
        );
        assert_eq!(1, Solution::longest_nice_subarray(vec![1]));
        assert_eq!(3, Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
        assert_eq!(1, Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]));
        assert_eq!(
            8,
            Solution::longest_nice_subarray(vec![
                84139415, 693324769, 614626365, 497710833, 615598711, 264, 65552, 50331652, 1,
                1048576, 16384, 544, 270532608, 151813349, 221976871, 678178917, 845710321,
                751376227, 331656525, 739558112, 267703680
            ])
        );
        assert_eq!(
            3,
            Solution::longest_nice_subarray(vec![
                744437702, 379056602, 145555074, 392756761, 560864007, 934981918, 113312475, 1090,
                16384, 33, 217313281, 117883195, 978927664
            ])
        );
    }
}
