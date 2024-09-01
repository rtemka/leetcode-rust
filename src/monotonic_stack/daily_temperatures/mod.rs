// https://leetcode.com/problems/daily-temperatures/description
struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut result = vec![0; temperatures.len()];

        for (i, &n) in temperatures.iter().enumerate().rev() {
            let peek = loop {
                if let Some(&e) = stack.last() {
                    if e.0 > n {
                        break e;
                    } else {
                        stack.pop();
                    }
                } else {
                    break (-1, temperatures.len());
                }
            };
            if peek.0 > 0 {
                result[i] = (peek.1 - i) as i32;
            };
            stack.push((n, i));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_temperatures() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }
}
