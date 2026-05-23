// https://leetcode.com/problems/number-of-senior-citizens/description
struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.iter().fold(0, |acc, x| {
            let b = x.as_bytes();
            acc + (b[11] > 0x36 || (b[11] == 0x36 && b[12] > 0x30)) as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_seniors() {
        assert_eq!(
            4,
            Solution::count_seniors(vec![
                "9751302862F0693".to_string(),
                "3888560693F7262".to_string(),
                "5485983835F0649".to_string(),
                "2580974299F6042".to_string(),
                "9976672161M6561".to_string(),
                "0234451011F8013".to_string(),
                "4294552179O6482".to_string(),
            ])
        );
        assert_eq!(
            2,
            Solution::count_seniors(vec![
                "7868190130M7522".to_string(),
                "7868190130M6022".to_string(),
                "7868190130M5922".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ])
        );
        assert_eq!(
            0,
            Solution::count_seniors(vec![
                "1313579440F2036".to_string(),
                "2921522980M5644".to_string(),
            ])
        );
    }
}
