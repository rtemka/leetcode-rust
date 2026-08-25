// #821
// https://leetcode.com/problems/shortest-distance-to-a-character/description/

struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut closest = s.len();
        let mut distances = vec![s.len() as i32; s.len()];

        for (i, cc) in s.char_indices() {
            closest = if cc == c { i } else { closest };
            distances[i] = i32::min(distances[i], usize::abs_diff(closest, i) as i32);
        }

        for (i, cc) in s.char_indices().rev() {
            closest = if cc == c { i } else { closest };
            distances[i] = i32::min(distances[i], usize::abs_diff(closest, i) as i32);
        }

        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_distance_to_a_character() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            Solution::shortest_to_char("loveleetcode".to_string(), 'e')
        );
    }
}
