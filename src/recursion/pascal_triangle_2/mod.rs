struct Solution {}

// https://leetcode.com/problems/pascals-triangle-ii/description/
impl Solution {
    // recursion variant
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let prev = Self::get_row(row_index - 1);
        let mut row = Vec::with_capacity(prev.len() + 1);
        let mut j = 0;
        while j <= prev.len() {
            row.push(if j > 0 && j < prev.len() {
                prev[j - 1] + prev[j]
            } else {
                1
            });
            j += 1;
        }
        row
    }

    pub fn get_row2(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let mut row = Vec::with_capacity(row_index as usize + 1);
        for _ in 0..row_index + 1 {
            // println!("{:?}", row);
            let mut prev = 0;
            let mut j = 0;
            let len = row.len();
            while j <= len {
                println!("{:?},{}", row, len);
                row.push(if j > 0 && j < len { prev + row[j] } else { 1 });
                prev = row[j];
                j += 1;
            }
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_triangle2() {
        assert_eq!(vec![1], Solution::get_row2(0));
        assert_eq!(vec![1, 1], Solution::get_row2(1));
        assert_eq!(vec![1, 2, 1], Solution::get_row(2));
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], Solution::get_row(5));
    }
}
