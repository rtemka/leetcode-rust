// https://leetcode.com/problems/count-the-number-of-complete-components/description
struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // Constraints:
        // 1 <= n <= 50

        // Init the map array and add connection to self
        let mut connections: Vec<u64> = (0..n)
            .map(|vertex| Self::add_connection(0, vertex as usize))
            .collect();
        for pair in &edges {
            // Destruct the pair
            let (vertexa, vertexb) = (pair[0] as usize, pair[1] as usize);
            // Add a -> b
            connections[vertexa] = Self::add_connection(connections[vertexa], vertexb);
            // Add a <- b
            connections[vertexb] = Self::add_connection(connections[vertexb], vertexa);
        }
        println!("{:?}", connections);
        // Here we should count amount of `standalone`(connected to no one) vertices.
        // Plus the amount of `complete` vertices(that would be the amount of verticies pointing
        // to each other; therefore they all must be equal)
        let mut count = 0;
        let mut memo = 0u64;
        for (i, bitmap) in connections.iter().enumerate() {
            println!("bitmap: {i} {:034b}", bitmap);
            println!("memo: {:034b}", memo);
            // Skip already checked
            if memo & (1 << i) > 0 {
                continue;
            }
            let mut bm = *bitmap as usize;
            let mut k = 0;
            let mut is_completed = true;
            while bm > 0 && is_completed {
                // When bit is set we check other vertex must be equal to this.
                // Or if this vertex is standalone it will be equal only to self.
                is_completed = bm & 1 == 0 || connections[k] == *bitmap;
                bm >>= 1;
                k += 1;
            }
            println!("the {i} vertex is_completed? {}", is_completed);
            // Add to count and to memo.
            count += is_completed as i32;
            memo |= *bitmap;
        }
        count
    }

    #[inline]
    fn add_connection(bitmap: u64, vertex: usize) -> u64 {
        // Constraint n <= 50 allows us to represent each
        // connection for the single vertex as the on/off bit.
        bitmap | (1 << vertex)
    }

    #[inline]
    fn is_standalone(bitmap: u64, n: usize) -> bool {
        // If vertex is not connected to other vertices
        // only the `self` bit would be on.
        bitmap == (1 << n)
    }

    #[inline]
    fn is_connected(bitmap: u64, vertex: usize) -> bool {
        bitmap & (1 << vertex) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_the_number_of_complete_components() {
        assert_eq!(
            0,
            Solution::count_complete_components(3, vec![vec![1, 0], vec![2, 1]])
        );

        assert_eq!(1, Solution::count_complete_components(1, vec![]));

        assert_eq!(1, Solution::count_complete_components(2, vec![vec![0, 1]]));

        assert_eq!(
            0,
            Solution::count_complete_components(
                4,
                vec![vec![1, 0], vec![2, 0], vec![2, 1], vec![3, 0]]
            )
        );

        assert_eq!(
            3,
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
            )
        );

        assert_eq!(
            1,
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            )
        );
    }
}
