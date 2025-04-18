// https://leetcode.com/problems/count-and-say/description
struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n < 2 {
            return "1".to_string();
        }
        let s = Self::count_and_say(n - 1);
        let mut res = String::new();
        let mut prev: char = '0';
        let mut count = 0;
        for c in s.chars() {
            if c != prev && count > 0 {
                res.push(char::from_digit(count as u32, 10).unwrap());
                res.push(prev);
                count = 0;
            }
            count += 1;
            prev = c;
        }
        if count > 0 {
            res.push(char::from_digit(count as u32, 10).unwrap());
            res.push(prev);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_say() {
        assert_eq!("1211".to_string(), Solution::count_and_say(4));
        assert_eq!("111221".to_string(), Solution::count_and_say(5));
        assert_eq!("312211".to_string(), Solution::count_and_say(6));
        assert_eq!(
            "311311222113111231131112132112311321322112111312211312111322212311322113212221"
                .to_string(),
            Solution::count_and_say(15)
        );
    }
}
