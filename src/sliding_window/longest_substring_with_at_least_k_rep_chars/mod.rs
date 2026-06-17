// #395
// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/description/

struct Solution;

// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/editorial/#approach-2-divide-and-conquer
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::longest_substring_helper(s.as_bytes(), &mut [0_usize; 26], 0, s.len(), k as usize)
            as i32
    }

    fn longest_substring_helper(
        s: &[u8],
        freqs: &mut [usize; 26],
        start: usize,
        end: usize,
        k: usize,
    ) -> usize {
        if end - start < k {
            return 0;
        }

        freqs.fill(0);

        for i in start..end {
            freqs[(s[i] - b'a') as usize] += 1;
        }

        for mid in start..end {
            if freqs[(s[mid] - b'a') as usize] >= k {
                continue;
            }

            let mut mid_next = mid + 1;
            while mid_next < end && freqs[(s[mid_next] - b'a') as usize] < k {
                mid_next += 1;
            }
            return usize::max(
                Self::longest_substring_helper(s, freqs, start, mid, k),
                Self::longest_substring_helper(s, freqs, mid_next, end, k),
            );
        }
        end - start
    }

    // https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/solutions/949552/python-sliding-window-solution-explained-3382
    pub fn longest_substring_sliding_approach(s: String, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let s = s.as_bytes();
        let mut freq = [0_usize; 26];

        let mut max_unique_chars_must_be_in_substring = 0;
        for c in s {
            let c = (c - b'a') as usize;
            if freq[c] == 0 {
                max_unique_chars_must_be_in_substring += 1;
            }
            freq[c] += 1;
        }

        for cnt_unique_chars_we_must_have_in_substring in 1..=max_unique_chars_must_be_in_substring
        {
            freq.fill(0);
            let (mut lo, mut hi) = (0, 0);
            let mut cnt_chars_with_frequency_1_or_more = 0;
            let mut cnt_chars_with_frequency_k_or_more = 0;

            while hi < s.len() {
                if cnt_chars_with_frequency_1_or_more <= cnt_unique_chars_we_must_have_in_substring
                {
                    let c = (s[hi] - b'a') as usize;
                    freq[c] += 1;
                    if freq[c] == 1 {
                        cnt_chars_with_frequency_1_or_more += 1;
                    }
                    if freq[c] == k {
                        cnt_chars_with_frequency_k_or_more += 1;
                    }
                    hi += 1;
                } else {
                    let c = (s[lo] - b'a') as usize;
                    lo += 1;
                    if freq[c] == k {
                        cnt_chars_with_frequency_k_or_more -= 1;
                    }
                    freq[c] -= 1;
                    if freq[c] == 0 {
                        cnt_chars_with_frequency_1_or_more -= 1;
                    }
                }
                if cnt_chars_with_frequency_1_or_more == cnt_unique_chars_we_must_have_in_substring
                    && cnt_chars_with_frequency_k_or_more
                        == cnt_unique_chars_we_must_have_in_substring
                {
                    result = result.max(hi - lo);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_substring_with_at_least_k_repeating_characters() {
        assert_eq!(3, Solution::longest_substring("aaabb".to_string(), 3));
        assert_eq!(5, Solution::longest_substring("ababbc".to_string(), 2));
        assert_eq!(1, Solution::longest_substring("a".to_string(), 1));
        assert_eq!(4, Solution::longest_substring("xababcabzab".to_string(), 2));
    }
}
