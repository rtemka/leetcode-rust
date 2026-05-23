// #787
// https://leetcode.com/problems/cheapest-flights-within-k-stops/description
struct Solution;

use std::collections::VecDeque;

impl Solution {
    // BFS Solution
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];

        for flight in flights.iter() {
            graph[flight[0] as usize].push((flight[1], flight[2]));
        }

        let mut queue = VecDeque::with_capacity(1);
        // cost, source, num of stops
        queue.push_back((0, src, 0));

        // costs to get to the i city from src
        let mut costs = vec![i32::MAX; n as usize];
        costs[src as usize] = 0;

        while let Some((cost, src, num_stops)) = queue.pop_front() {
            for &(dest, price) in &graph[src as usize] {
                let new_cost = cost + price;
                if new_cost < costs[dest as usize] && num_stops <= k {
                    queue.push_back((new_cost, dest, num_stops + 1));
                    costs[dest as usize] = new_cost;
                }
            }
        }
        match costs[dst as usize] {
            i32::MAX => -1,
            res => res,
        }
    }

    // Bellman-Ford
    pub fn find_cheapest_price_bf(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let (src, dst, n) = (src as usize, dst as usize, n as usize);
        let mut costs = vec![i32::MAX; n];
        costs[src] = 0;

        let mut flight_costs = vec![0; n];
        for _ in 0..=k {
            flight_costs.copy_from_slice(&costs);
            for flight in &flights {
                let (from, to, price) = (flight[0] as usize, flight[1] as usize, flight[2]);
                if costs[from] != i32::MAX {
                    flight_costs[to] = i32::min(flight_costs[to], costs[from] + price);
                }
            }
            costs.copy_from_slice(&flight_costs);
        }
        if costs[dst] == i32::MAX {
            -1
        } else {
            costs[dst]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cheapest_flights_within_k_stops() {
        assert_eq!(
            6,
            Solution::find_cheapest_price(
                4,
                vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
                0,
                3,
                1
            )
        );
        assert_eq!(
            700,
            Solution::find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        );

        assert_eq!(
            -1,
            Solution::find_cheapest_price(
                5,
                vec![
                    vec![4, 1, 1],
                    vec![1, 2, 3],
                    vec![0, 3, 2],
                    vec![0, 4, 10],
                    vec![3, 1, 1],
                    vec![1, 4, 3]
                ],
                2,
                1,
                1
            )
        );
    }

    #[test]
    fn cheapest_flights_within_k_stops_bellman_ford() {
        assert_eq!(
            6,
            Solution::find_cheapest_price_bf(
                4,
                vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
                0,
                3,
                1
            )
        );
        assert_eq!(
            700,
            Solution::find_cheapest_price_bf(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        );

        assert_eq!(
            -1,
            Solution::find_cheapest_price_bf(
                5,
                vec![
                    vec![4, 1, 1],
                    vec![1, 2, 3],
                    vec![0, 3, 2],
                    vec![0, 4, 10],
                    vec![3, 1, 1],
                    vec![1, 4, 3]
                ],
                2,
                1,
                1
            )
        );
    }
}
