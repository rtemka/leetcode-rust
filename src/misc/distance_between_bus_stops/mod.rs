// #1184
// https://leetcode.com/problems/distance-between-bus-stops/description/

struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (start, destination) = (
            i32::min(start, destination) as usize,
            i32::max(start, destination) as usize,
        );

        let clockwise = distance[start..destination].iter().sum::<i32>();
        let counterclockwise =
            distance[destination..].iter().sum::<i32>() + distance[..start].iter().sum::<i32>();

        i32::min(clockwise, counterclockwise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_bus_stops() {
        assert_eq!(
            4,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3)
        );

        assert_eq!(
            3,
            Solution::distance_between_bus_stops(vec![7, 6, 3, 0, 3], 0, 4)
        );
    }
}
