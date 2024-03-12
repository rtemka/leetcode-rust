struct Solution {}

impl Solution {
    pub fn get_row(_row_index: i32) -> Vec<i32> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_triangle2() {
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1], Solution::get_row(0));
        assert_eq!(vec![1, 1], Solution::get_row(1));
    }
}
