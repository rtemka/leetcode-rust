// https://leetcode.com/problems/number-of-provinces/description
struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut marked: Vec<bool> = vec![false; is_connected.len()];
        let mut count = 0;
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..is_connected.len() {
            if marked[i] {
                continue;
            }
            Self::dfs(&is_connected, &mut marked, &mut stack, i);
            count += 1;
        }
        count
    }

    #[inline]
    fn dfs(cities: &Vec<Vec<i32>>, marked: &mut Vec<bool>, stack: &mut Vec<usize>, city: usize) {
        marked[city] = true;
        stack.push(city);
        while let Some(city) = stack.pop() {
            for (idx, &edge) in cities[city].iter().enumerate() {
                if edge == 0 || marked[idx] {
                    continue;
                }
                marked[idx] = true;
                stack.push(idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_provinces() {
        assert_eq!(
            2,
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
        );

        assert_eq!(
            2,
            Solution::find_circle_num(vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1]
            ])
        );

        assert_eq!(
            3,
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
        );
    }
}
