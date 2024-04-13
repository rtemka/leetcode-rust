// https://leetcode.com/problems/pascals-triangle/description/
struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::with_capacity(num_rows as usize);
        triangle.push(vec![1]);
        for r in 1..num_rows as usize {
            let prev_row = &triangle[r - 1];
            let mut res = Vec::with_capacity(prev_row.len() + 1);
            for i in 0..=prev_row.len() {
                res.push(if i == 0 || i == prev_row.len() {
                    1
                } else {
                    prev_row[i] + prev_row[i - 1]
                });
            }
            triangle.push(res);
        }
        triangle
    }

    pub fn generate_rec(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 1 {
            return vec![vec![1]];
        }
        let mut triangle = Self::generate_rec(num_rows - 1);
        let prev_row = &triangle[triangle.len() - 1];
        let mut res = Vec::with_capacity(prev_row.len() + 1);
        let mut i = 0;
        while i < prev_row.len() + 1 {
            res.push(match i {
                0 => 1,
                _ if i == prev_row.len() => 1,
                _ => prev_row[i] + prev_row[i - 1],
            });
            i += 1;
        }
        triangle.push(res);
        println!("{:?}", triangle);
        triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pascal_triangle() {
        assert_eq!(vec![vec![1]], Solution::generate(1));
        assert_eq!(
            vec![vec![1], vec![1, 1], vec![1, 2, 1]],
            Solution::generate(3)
        );
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            Solution::generate(5)
        );
    }
}
