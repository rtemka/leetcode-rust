use std::collections::HashMap;

// https://leetcode.com/problems/minimum-index-sum-of-two-lists/description/
struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let (short, long) = if list1.len() > list2.len() {
            (list2, list1)
        } else {
            (list1, list2)
        };

        let mut v = Vec::with_capacity(2); // only two min index sum is possible.
        let mut min = 0;
        let map = short
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<&String, usize>>();

        for (x_idx, x_str) in long.into_iter().enumerate() {
            if x_idx > min && v.len() > 0 {
                return v;
            }

            match map.get(&x_str) {
                Some(y_idx) if y_idx + x_idx <= min || v.len() == 0 => {
                    if y_idx + x_idx < min {
                        v.truncate(0);
                    }
                    min = y_idx + x_idx;
                    v.push(x_str);
                }
                _ => continue,
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_restaurant() {
        assert_eq!(
            vec!["Shogun".to_string()],
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            )
        );

        assert_eq!(
            vec!["Shogun".to_string()],
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            )
        );

        assert_eq!(
            vec!["sad".to_string(), "happy".to_string()],
            Solution::find_restaurant(
                vec!["happy".to_string(), "sad".to_string(), "good".to_string(),],
                vec!["sad".to_string(), "happy".to_string(), "good".to_string(),],
            )
        );
    }
}
