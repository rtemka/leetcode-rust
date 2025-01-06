use std::{collections::HashSet, iter};

// https://leetcode.com/problems/permutation-sequence/description/
struct Solution;

impl Solution {
    // ASCII offset of numbers
    const OFFSET: u8 = 0x30;

    pub fn get_permutation_backtrack(n: i32, k: i32) -> String {
        let mut s = String::with_capacity(n as usize);
        let mut set: HashSet<u8> = HashSet::with_capacity(n as usize);
        let mut v: Vec<String> = Vec::with_capacity(k as usize);

        Self::backtrack(&mut v, &mut s, &mut set, n as usize, k as usize);
        // dbg!(&v);

        unsafe { v.pop().unwrap_unchecked() }
    }

    #[inline]
    fn backtrack(v: &mut Vec<String>, s: &mut String, set: &mut HashSet<u8>, n: usize, k: usize) {
        if s.len() == n {
            v.push(s.clone());
            return;
        }
        for i in 1..=n as u8 {
            if v.len() == v.capacity() {
                return;
            }
            if !set.insert(i) {
                continue;
            }
            s.push((i + Self::OFFSET) as char);
            Self::backtrack(v, s, set, n, k);
            s.pop();
            set.remove(&i);
        }
    }

    pub fn get_permutation(n: i32, k: i32) -> String {
        // calculate starting point close to the k-th sequence
        let (k, start) = {
            let combinations_in_round = (1..=n).product::<i32>() / n;
            let (mut k_start, mut start) = (1, 1);
            while k_start + combinations_in_round <= k {
                k_start += combinations_in_round;
                start += 1;
            }
            (k - k_start, start)
        };
        let mut seq: Vec<u8> = iter::once(start)
            .chain((1..=n as u8).filter(|x| *x != start))
            .collect();

        // dbg!(&seq);
        // dbg!(k);
        (0..k).for_each(|_| Self::next_permutation(&mut seq));
        String::from_iter(seq.into_iter().map(|x| (x + Self::OFFSET) as char))
    }

    #[inline]
    fn next_permutation(nums: &mut Vec<u8>) {
        // Алгоритм Нарайаны для поиска следующей комбинации
        // https://ru.wikipedia.org/wiki/%D0%90%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC_%D0%9D%D0%B0%D1%80%D0%B0%D0%B9%D0%B0%D0%BD%D1%8B
        //
        // Этап 1: Найти наибольшее i, для которого выполняется условие nums[i-1] >= nums[i]
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        // Этап 2: Найти наибольшее j > i, для которого выполняется условие nums[i-1] >= nums[j]
        let mut j = nums.len() - 1;
        while j > i && nums[i.saturating_sub(1)] >= nums[j] {
            j -= 1;
        }
        // поменять местами
        nums.swap(i.saturating_sub(1), j);
        j = nums.len() - 1;
        // развернуть все элементы после i
        while i < j {
            nums.swap(i, j);
            (i, j) = (i + 1, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_sequence() {
        // println!("{}", (1..=9).product::<usize>() / 9);
        assert_eq!(String::from("1"), Solution::get_permutation(1, 1));
        assert_eq!(String::from("213"), Solution::get_permutation(3, 3));
        assert_eq!(String::from("2314"), Solution::get_permutation(4, 9));
        assert_eq!(String::from("123"), Solution::get_permutation(3, 1));
    }
}
