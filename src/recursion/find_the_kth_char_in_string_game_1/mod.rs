// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/description/
struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let k = k as usize;
        let mut letters: Vec<u8> = Vec::with_capacity(k.ilog2() as usize);
        letters.push(b'a');
        while letters.len() <= k {
            let l = letters.len();
            for i in 0..l {
                letters.push(if letters[i] == b'z' {
                    b'a'
                } else {
                    letters[i] + 1
                });
            }
            // println!("{:?}", String::from_utf8_lossy(&letters));
        }

        letters[k - 1] as char
    }

    pub fn kth_character_rec(k: i32) -> char {
        let mut letters: Vec<u8> = Vec::with_capacity(k.ilog2() as usize);
        letters.push(b'a');
        Self::kth_character_rec_helper(&mut letters, k as usize);
        letters[k as usize - 1] as char
    }

    fn kth_character_rec_helper(letters: &mut Vec<u8>, k: usize) {
        if letters.len() > k {
            return;
        }
        let l = letters.len();
        for i in 0..l {
            letters.push(if letters[i] == b'z' {
                b'a'
            } else {
                letters[i] + 1
            });
        }
        Self::kth_character_rec_helper(letters, k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_the_kth_char_in_string_game() {
        assert_eq!('a', Solution::kth_character(1));
        assert_eq!('b', Solution::kth_character(5));
        assert_eq!('c', Solution::kth_character(10));
        assert_eq!('h', Solution::kth_character(500));

        assert_eq!('b', Solution::kth_character_rec(5));
        assert_eq!('c', Solution::kth_character_rec(10));
    }
}
