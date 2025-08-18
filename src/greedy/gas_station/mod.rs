// #134
// https://leetcode.com/problems/gas-station/description/
struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // If the total gas is less than the total cost, completing the circuit is impossible.
        if gas.iter().sum::<i32>() < cost.iter().sum() {
            return -1;
        }
        let mut start = 0;
        let mut tank = 0;
        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            // If our tank has negative gas, we cannot continue through the
            // circuit from the current start point, nor from any station
            // before or including the current station 'i'.
            if tank < 0 {
                // Set the next station as the new start point and reset the tank.
                start = i + 1;
                tank = 0;
            }
        }
        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gas_station() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
    }
}
