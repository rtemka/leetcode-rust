// https://leetcode.com/problems/asteroid-collision/description
struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(asteroids.len() / 2);
        for hi in asteroids {
            loop {
                match stack.pop() {
                    None => {
                        stack.push(hi);
                        break;
                    }
                    Some(lo) if lo > 0 && hi < 0 => {
                        if lo + hi > 0 {
                            stack.push(lo);
                        }
                        if lo + hi >= 0 {
                            break;
                        }
                    }
                    Some(lo) => {
                        stack.push(lo);
                        stack.push(hi);
                        break;
                    }
                }
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asteroid_collision() {
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
    }
}
