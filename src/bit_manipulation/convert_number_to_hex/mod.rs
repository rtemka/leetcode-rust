// https://leetcode.com/problems/convert-a-number-to-hexadecimal/description/
struct Solution;

impl Solution {
    pub fn to_hex_cheat(num: i32) -> String {
        format!("{:x}", num)
    }

    pub fn to_hex(mut num: i32) -> String {
        // const BASE: [char; 16] = [
        //     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        // ];
        let mask = 0xf;
        let mut result = Vec::<char>::new();
        let quad = 4;
        for _ in (4..=32).step_by(quad) {
            result.push(match num & mask {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                _ => 'f',
            });
            num >>= quad;
            if num == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex() {
        assert_eq!("1a".to_string(), Solution::to_hex(26));
        assert_eq!("ffffffff".to_string(), Solution::to_hex(-1));
    }
}
