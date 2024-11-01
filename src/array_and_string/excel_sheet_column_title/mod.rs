// https://leetcode.com/problems/excel-sheet-column-title/description/
struct Solution;

const BASE: i32 = 26;
const CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut title = String::new();
        while column_number > 0 {
            column_number -= 1;
            title.push(CHARS[(column_number % 26) as usize]);
            column_number /= BASE;
        }

        title.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_title() {
        assert_eq!("AZ".to_owned(), Solution::convert_to_title(52));
        assert_eq!("A".to_owned(), Solution::convert_to_title(1));
        assert_eq!("AB".to_owned(), Solution::convert_to_title(28));
        assert_eq!("ZY".to_owned(), Solution::convert_to_title(701));
        assert_eq!("ALL".to_owned(), Solution::convert_to_title(1000));
        assert_eq!("GZX".to_owned(), Solution::convert_to_title(5432));
        assert_eq!("AYTD".to_owned(), Solution::convert_to_title(35000));
    }
}
