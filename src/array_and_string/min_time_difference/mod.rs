// https://leetcode.com/problems/minimum-time-difference/description
struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes: Vec<i32> = time_points
            .iter()
            .map(|point| {
                let (h, m) = point.split_at(2);
                Self::parse_clock_digit(h) * 60 + Self::parse_clock_digit(&m[1..])
            })
            .collect();

        minutes.sort_unstable();

        let mut min_interval = 1440 - minutes[minutes.len() - 1] + minutes[0];
        for i in 1..minutes.len() {
            min_interval = min_interval.min(minutes[i] - minutes[i - 1]);
        }

        min_interval
    }

    #[inline(always)]
    fn parse_clock_digit(clock_str: &str) -> i32 {
        unsafe {
            if clock_str.chars().nth(0).unwrap_unchecked() == '0' {
                clock_str[1..].parse().unwrap_unchecked()
            } else {
                clock_str.parse().unwrap_unchecked()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_min_time_diff() {
        assert_eq!(
            1,
            Solution::find_min_difference(vec!["23:59".into(), "00:00".into()])
        );
        assert_eq!(
            0,
            Solution::find_min_difference(vec!["00:00".into(), "23:59".into(), "00:00".into()])
        );
        assert_eq!(
            2,
            Solution::find_min_difference(vec![
                "00:01".into(),
                "09:18".into(),
                "09:20".into(),
                "23:40".into()
            ])
        );
    }
}
