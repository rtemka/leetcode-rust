struct Solution;

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        if arrival_time + delayed_time >= 24 {
            arrival_time + delayed_time - 24
        } else {
            arrival_time + delayed_time
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_delayed_arrival_time() {
        assert_eq!(23, Solution::find_delayed_arrival_time(23, 24));
        assert_eq!(23, Solution::find_delayed_arrival_time(23, 24));
        assert_eq!(20, Solution::find_delayed_arrival_time(20, 24));
        assert_eq!(20, Solution::find_delayed_arrival_time(15, 5));
        assert_eq!(0, Solution::find_delayed_arrival_time(13, 11));
    }
}
