// #3633
// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i
// See also: #3635

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        mut land_start_time: Vec<i32>,
        mut land_duration: Vec<i32>,
        mut water_start_time: Vec<i32>,
        mut water_duration: Vec<i32>,
    ) -> i32 {
        let mut land_earliest = (0, i32::MAX);
        let mut land_ride: Vec<_> = land_start_time
            .drain(..)
            .zip(land_duration.drain(..))
            .inspect(|&(start, end)| {
                if start + end < land_earliest.0 + land_earliest.1 {
                    land_earliest = (start, end);
                }
            })
            .collect();

        let mut water_earliest = (0, i32::MAX);
        let mut water_ride: Vec<_> = water_start_time
            .drain(..)
            .zip(water_duration.drain(..))
            .inspect(|&(start, end)| {
                if start + end < water_earliest.0 + water_earliest.1 {
                    water_earliest = (start, end);
                }
            })
            .collect();

        water_ride.sort_unstable_by_key(|k| i32::max(land_earliest.0 + land_earliest.1, k.0) + k.1);

        let land_first_earliest =
            i32::max(land_earliest.0 + land_earliest.1, water_ride[0].0) + water_ride[0].1;

        land_ride
            .sort_unstable_by_key(|k| i32::max(water_earliest.0 + water_earliest.1, k.0) + k.1);

        let water_first_earliest =
            i32::max(water_earliest.0 + water_earliest.1, land_ride[0].0) + land_ride[0].1;

        i32::min(land_first_earliest, water_first_earliest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earliest_finish_time_for_land_and_water_rides_1() {
        assert_eq!(
            98,
            Solution::earliest_finish_time(vec![37, 40], vec![53, 74], vec![20, 84], vec![41, 8])
        );

        assert_eq!(
            98,
            Solution::earliest_finish_time(vec![31, 8], vec![47, 64], vec![3, 7], vec![95, 44])
        );

        assert_eq!(
            209,
            Solution::earliest_finish_time(vec![79], vec![51], vec![99], vec![79])
        );

        assert_eq!(
            9,
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3])
        );

        assert_eq!(
            14,
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10])
        );

        assert_eq!(
            12,
            Solution::earliest_finish_time(vec![5, 9], vec![3, 1], vec![1], vec![10])
        );

        assert_eq!(
            12,
            Solution::earliest_finish_time(vec![5, 9], vec![3, 1], vec![1], vec![10])
        );

        assert_eq!(
            13,
            Solution::earliest_finish_time(vec![5, 9], vec![2, 2], vec![1], vec![10])
        );
    }
}
