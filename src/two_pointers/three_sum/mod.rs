use std::collections::HashSet;

// https://leetcode.com/problems/3sum/description/
struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("\n\n");
        nums.sort_unstable();
        let mut triplet = [0; 3];
        let mut set = HashSet::<[i32; 3]>::new();

        for (i, &x) in nums.iter().enumerate() {
            println!("{:?};set{:?}", nums, set);
            for (j, &y) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                let z = {
                    let target = -(x + y);
                    let mut zi = nums.partition_point(|&x| x < target);
                    while zi < nums.len() && nums[zi] == target {
                        if zi != i && zi != j {
                            break;
                        }
                        zi += 1;
                    }
                    if zi == nums.len() || nums[zi] != target {
                        continue;
                    }
                    nums[zi]
                };
                (triplet[0], triplet[1], triplet[2]) = (x, y, z);
                triplet.sort_unstable();
                set.insert(triplet);
            }
        }

        set.into_iter().map(|triplet| triplet.to_vec()).collect()
    }

    pub fn three_sum_best(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort_unstable();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum() {
        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());

        let mut left = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        left.sort();
        let mut right = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        right.sort();
        assert_eq!(left, right);

        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![0, 0, 0]));

        let mut left = vec![vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]];
        left.sort();
        let mut right = Solution::three_sum(vec![3, 0, -2, -1, 1, 2]);
        right.sort();
        assert_eq!(left, right);
    }
}
