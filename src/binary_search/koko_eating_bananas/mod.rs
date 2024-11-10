// https://leetcode.com/problems/koko-eating-bananas
struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut lo, mut hi) = (1, *piles.iter().max().unwrap());
        while lo < hi {
            let mid = (lo.saturating_add(hi)) >> 1;
            let speed = piles.iter().fold(0, |acc, &e| acc + (e + mid - 1) / mid); // ceil division
            if speed <= h {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_eating_speed() {
        assert_eq!(4, Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    }
}
