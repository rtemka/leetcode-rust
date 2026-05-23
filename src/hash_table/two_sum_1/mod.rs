use std::collections::HashMap;

/// O(n2) solution.
pub fn two_sum_on2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        if nums[i] > target {
            continue;
        }
        if nums[i] == target {
            return vec![i as i32];
        }
        for j in i..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::new()
}

/// O(n) solution.
pub fn two_sum_on(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        map.insert(nums[i], i);
    }
    for i in 0..nums.len() {
        if let Some(v) = map.get(&(target - nums[i])) {
            if *v == i {
                continue;
            }
            return vec![*v as i32, i as i32];
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_on2() {
        assert!(vec![0, 1] == two_sum_on2(vec![2, 7, 11, 15], 9));
        assert!(vec![0, 3] == two_sum_on2(vec![2, 7, 11, 15], 17));
    }

    #[test]
    fn test_two_sum_on() {
        let mut first_case = two_sum_on(vec![2, 7, 11, 15], 9);
        first_case.sort();
        assert!(vec![0, 1] == first_case);

        let mut second_case = two_sum_on(vec![2, 7, 11, 15], 17);
        second_case.sort();
        assert!(vec![0, 3] == second_case);

        let mut third_case = two_sum_on(vec![3, 2, 4], 6);
        third_case.sort();
        assert!(vec![1, 2] == third_case);
    }
}
