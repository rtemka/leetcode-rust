// https://leetcode.com/problems/next-permutation/description/
struct Solution;

impl Solution {
    #[allow(clippy::all)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
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
    fn next_permutation() {
        let mut v = vec![1, 2, 3];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 3, 2], v);

        let mut v = vec![3, 2, 1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 2, 3], v);

        let mut v = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![0, 1, 2, 3, 5, 1, 2, 3, 4, 4], v);

        let mut v = vec![8, 11, 3];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![11, 3, 8], v);
    }
}
