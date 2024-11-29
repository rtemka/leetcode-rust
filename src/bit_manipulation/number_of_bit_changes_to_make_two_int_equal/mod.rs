// https://leetcode.com/problems/number-of-bit-changes-to-make-two-integers-equal/description/
struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        // находим различные биты с помощью XOR,
        // затем с помощью AND получаем 1,
        // которые надо поменять на 0.
        // Наконец, сравниваем с различающимися
        // битами и если это они и есть, то считаем единички.
        let changes = (n ^ k) & n;
        if changes == (n ^ k) {
            changes.count_ones() as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_changes() {
        assert_eq!(2, Solution::min_changes(13, 4));
        assert_eq!(0, Solution::min_changes(21, 21));
        assert_eq!(-1, Solution::min_changes(14, 13));
    }
}
