// #492
// https://leetcode.com/problems/construct-the-rectangle/

struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut root = area.isqrt();

        while area % root != 0 {
            root -= 1;
        }

        vec![area / root, root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_the_rectangle() {
        assert_eq!(vec![2, 2], Solution::construct_rectangle(4));

        assert_eq!(vec![427, 286], Solution::construct_rectangle(122122));
    }
}
