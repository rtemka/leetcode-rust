// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/description
struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut v = Vec::with_capacity(spells.len());
        potions.sort_unstable();
        for n in spells {
            let n = n as i64;
            let p = match (success / n, success % n) {
                (0, _) => 1,     // if the n is bigger then success
                (n, 0) => n,     // if the success can be evenly devided
                (n, _) => n + 1, // if there is a remainder
            };
            let pp = potions.partition_point(|&x| (x as i64/* the tricky bit */) < p);
            // println!("n={};p={};potions={:?};pp={}", n, p, potions, pp);
            if pp == potions.len() {
                v.push(0);
            } else {
                v.push((potions.len() - pp) as i32);
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_pairs() {
        assert_eq!(
            vec![4, 0, 3],
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
        );
        assert_eq!(
            vec![2, 0, 2],
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16)
        );
    }
}
