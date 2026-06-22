// #806
// https://leetcode.com/problems/number-of-lines-to-write-string/description/

struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 0;
        let mut current_line = 0;
        for c in s.as_bytes() {
            let width = widths[(c - b'a') as usize];
            if current_line + width > 100 {
                lines += 1;
                current_line = width;
            } else {
                current_line += width;
            }
        }

        vec![lines + (current_line > 0) as i32, current_line]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_lines_to_write_string() {
        assert_eq!(
            vec![3, 60],
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            )
        );

        assert_eq!(
            vec![2, 4],
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            )
        );
    }
}
