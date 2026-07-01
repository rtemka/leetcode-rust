// #2444
// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/count-subarrays-with-fixed-bounds/solutions/2708099/javacpython-sliding-window-with-explanat-f2wf/
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0;
        let (mut j_min, mut j_max, mut j_invalid) = (-1, -1, -1);

        println!("{:?}, min_k={}, max_k={}", nums, min_k, max_k);

        for (i, n) in nums.into_iter().enumerate() {
            if n < min_k || n > max_k {
                j_invalid = i as i64;
            }

            if n == min_k {
                j_min = i as i64;
            }

            if n == max_k {
                j_max = i as i64;
            }

            res += i64::max(0, i64::min(j_min, j_max) - j_invalid);

            println!(
                "jmin={}, jmax={}, jbad={}, {}=max(0, min({}, {}) - {})",
                j_min,
                j_max,
                j_invalid,
                i64::max(0, i64::min(j_min, j_max) - j_invalid),
                j_min,
                j_max,
                j_invalid,
            );
        }

        res
    }

    // 1. Split nums so that the resulting arrays only contain values that satisfy minK <= value <= maxK
    // (split at indices of values that do not), hereinafter referred to as partial input.
    //
    // For each of those partial inputs:
    //
    // 2. Count subarrays that do not contain both minK and maxK at the same time
    // and subtract the result from the total number of subarrays in your partial input (of length n), which is
    // ((n * (n + 1)) // 2)
    //
    // Finally, sum up the partial results.
    pub fn count_subarrays_my(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        let (mut lo, mut hi) = (0, 0);
        let (mut has_min, mut has_max) = (false, false);

        println!("{:?}, min_k={}, max_k={}", nums, min_k, max_k);

        while hi <= nums.len() {
            has_min = has_min.max(hi < nums.len() && nums[hi] == min_k);
            has_max = has_max.max(hi < nums.len() && nums[hi] == max_k);

            let was_valid = has_min && has_max;
            let now_broken = hi == nums.len() || nums[hi] > max_k || nums[hi] < min_k;

            println!(
                "lo={}; hi={}\thas min={}; has max={}; was_valid={}; now_broken={}",
                lo, hi, has_min, has_max, was_valid, now_broken
            );

            if was_valid && now_broken {
                let n = hi - lo;
                result += (n * (n + 1) / 2) as i64
                    - Self::count_subarrays_excluding_min_and_max_k(&nums[lo..hi], min_k, max_k);
            }
            if now_broken {
                lo = hi + 1;
                (has_min, has_max) = (false, false);
            }
            hi += 1;
        }

        result
    }

    #[inline]
    fn count_subarrays_excluding_min_and_max_k(nums: &[i32], min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        let (mut lo, mut hi) = (0, 0);
        let (mut min_i, mut max_i) = (0, 0);

        while hi < nums.len() {
            min_i = Self::min_index(nums, min_i, hi);
            max_i = Self::max_index(nums, max_i, hi);

            let invalidated = nums[min_i] == min_k && nums[max_i] == max_k;
            if invalidated {
                lo = usize::min(min_i, max_i) + 1;
            }

            result += if hi >= lo { hi - lo + 1 } else { 0 };

            hi += 1;
        }

        println!(
            "{:?} count_of_subarrays_excluding_min_and_max_k={}",
            nums, result
        );

        result as i64
    }

    #[inline]
    fn min_index(nums: &[i32], a: usize, b: usize) -> usize {
        if nums[b] <= nums[a] { b } else { a }
    }

    #[inline]
    fn max_index(nums: &[i32], a: usize, b: usize) -> usize {
        if nums[b] >= nums[a] { b } else { a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_subarrays_with_fixed_bounds() {
        assert_eq!(1, Solution::count_subarrays(vec![4, 3], 3, 3));

        assert_eq!(
            18,
            Solution::count_subarrays(vec![1, 2, 3, 4, 1, 2, 3, 4], 1, 4)
        );

        assert_eq!(1, Solution::count_subarrays(vec![1, 1000000], 1, 1000000));

        assert_eq!(
            1,
            Solution::count_subarrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1, 10)
        );

        assert_eq!(2, Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5));

        assert_eq!(
            4,
            Solution::count_subarrays(vec![1, 1, 3, 5, 2, 7, 5], 1, 5)
        );

        assert_eq!(10, Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1));
        assert_eq!(3, Solution::count_subarrays(vec![1, 1], 1, 1));
        assert_eq!(0, Solution::count_subarrays(vec![6, 6], 1, 1));
    }
}
