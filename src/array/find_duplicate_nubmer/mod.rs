// https://leetcode.com/problems/find-the-duplicate-number/description/
struct Solution;

impl Solution {
    // You must solve the problem without modifying
    // the array nums and using only constant extra space.
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Алгоритм Флойда-Уоршалла с быстрым и медленным указателями
        // для поиска циклов.
        let (mut slow, mut fast) = (nums[0] as usize, nums[nums[0] as usize] as usize);
        // находим цикл: быстрый указатель зациклится на одном из
        // дубликатов
        while slow != fast {
            let (prev_slow, prev_fast) = (slow, fast);
            (slow, fast) = (nums[slow] as usize, nums[nums[fast] as usize] as usize);
            println!(
                "slow: {} => {}\t fast: {} => {}",
                prev_slow, slow, prev_fast, fast
            );
            println!("");
        }
        // Далее отправляем еще один указатель сначала
        // Он догонит зацикленный указатель ровно на дубликате
        // Магия не иначе
        let mut slow2 = 0;
        while slow != slow2 {
            let (prev_slow, prev_slow2) = (slow, slow2);
            (slow, slow2) = (nums[slow] as usize, nums[slow2] as usize);
            println!(
                "slow: {} => {}\t slow2: {} => {}",
                prev_slow, slow, prev_slow2, slow2
            );
            println!("");
        }
        slow2 as i32
    }

    pub fn find_duplicate_cheat(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = 1;
        while i < nums.len() {
            if nums[i - 1] == nums[i] {
                return nums[i];
            }
            i += 1;
        }
        unreachable!("loop should always return")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_duplicate_number() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    }
}
