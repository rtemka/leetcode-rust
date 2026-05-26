// #292
// https://leetcode.com/problems/nim-game/description/
// Some explanation: https://leetcode.com/problems/nim-game/description/comments/1569342/

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nim_game() {
        assert!(!Solution::can_win_nim(4));
    }
}
