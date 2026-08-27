// #2904
// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string

struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let s = s.as_bytes();

        let mut shortest: &[u8] = "".as_bytes();
        let mut lo = 0;
        let mut ones = 0;
        for (hi, &c) in s.iter().enumerate() {
            ones += (c == b'1') as i32;

            while lo < hi && ones > k {
                ones -= (s[lo] == b'1') as i32;
                lo += 1;
            }

            while lo <= hi && ones == k {
                if shortest.is_empty()
                    || hi - lo + 1 < shortest.len()
                    || (hi - lo + 1 == shortest.len() && s[lo..=hi].lt(shortest))
                {
                    shortest = &s[lo..=hi];
                }
                ones -= (s[lo] == b'1') as i32;
                lo += 1;
            }
        }

        String::from_utf8_lossy(shortest).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_lexicographically_smallest_beautiful_string() {
        assert_eq!(
            "1100100101011".to_string(),
            Solution::shortest_beautiful_substring("1100100101011001001".to_string(), 7)
        );

        assert_eq!(
            "11001".to_string(),
            Solution::shortest_beautiful_substring("100011001".to_string(), 3)
        );
    }
}
