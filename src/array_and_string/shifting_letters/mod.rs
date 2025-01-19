// https://leetcode.com/problems/shifting-letters/description/
struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        //  Constraints:
        //
        // 1 <= s.length <= 10^5
        // s consists of lowercase English letters.
        // shifts.length == s.length

        // Compute prefix sum for shifts array.
        let prefix_sum: Vec<usize> = shifts
            .into_iter()
            .scan(0usize, |sum, x| {
                *sum += x as usize;
                Some(*sum)
            })
            .collect();
        // println!("{:?}", prefix_sum);
        // In order to calculate right shift amount for each letter
        // we must take total_sum(which will be the last element of prefix_sum)
        // and subtract letter position - 1 from prefix_sum array.
        // After we just shift letter for the right amount of `places`.
        let last = prefix_sum[prefix_sum.len() - 1];
        s.chars()
            .enumerate()
            // .inspect(|(i, c)| println!("{i}, {c};shift={}", last - prefix_sum[i.saturating_sub(1)]))
            .map(|(i, c)| {
                Self::shift_letter(
                    c,
                    if i > 0 {
                        last - prefix_sum[i - 1]
                    } else {
                        last
                    },
                )
            })
            .collect()
    }

    #[inline]
    fn shift_letter(letter: char, shift: usize) -> char {
        let shift = (shift % 26) as u8;
        let shifted = letter as u8 + shift;
        // println!("letter={letter};shift={shift};shifted={shifted}");
        if shifted > 122 {
            (96 + (shifted - 122)) as char
        } else {
            shifted as char
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifting_letters() {
        assert_eq!(
            "wqqwlcjnkphhsyvrkdod".to_string(),
            Solution::shifting_letters(
                "mkgfzkkuxownxvfvxasy".to_string(),
                vec![
                    505870226, 437526072, 266740649, 224336793, 532917782, 311122363, 567754492,
                    595798950, 81520022, 684110326, 137742843, 275267355, 856903962, 148291585,
                    919054234, 467541837, 622939912, 116899933, 983296461, 536563513
                ]
            )
        );
        assert_eq!(
            "rpl".to_string(),
            Solution::shifting_letters("abc".to_string(), vec![3, 5, 9])
        );
        assert_eq!(
            "lmz".to_string(),
            Solution::shifting_letters("acz".to_string(), vec![27, 10, 26])
        );
    }
}
