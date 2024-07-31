use std::collections::VecDeque;

// https://leetcode.com/problems/dota2-senate/description
struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let (mut dire_ban, mut radiant_ban) = (0, 0);
        let mut q = VecDeque::with_capacity(senate.len() / 2);
        for c in senate.chars() {
            Self::vote(c, &mut q, &mut dire_ban, &mut radiant_ban);
        }
        // println!("{:?},dire-ban={};radiant-ban={}", q, dire_ban, radiant_ban);
        while dire_ban < q.len() && radiant_ban < q.len() {
            // println!(
            //     "senate={}\t{:?},dire-ban={};radiant-ban={}",
            //     senate, q, dire_ban, radiant_ban
            // );
            if let Some(c) = q.pop_front() {
                Self::vote(c, &mut q, &mut dire_ban, &mut radiant_ban);
            }
        }
        if radiant_ban > 0 {
            // println!("senate={}\t Dire wins!", senate);
            "Dire".to_string()
        } else {
            // println!("senate={}\t Radiant wins!", senate);
            "Radiant".to_string()
        }
    }

    #[inline]
    fn vote(c: char, q: &mut VecDeque<char>, dire_ban: &mut usize, radiant_ban: &mut usize) {
        match c {
            'R' if *radiant_ban == 0 => {
                q.push_back(c);
                *dire_ban += 1;
            }
            'R' => *radiant_ban -= 1,
            'D' if *dire_ban == 0 => {
                q.push_back(c);
                *radiant_ban += 1;
            }
            'D' => *dire_ban -= 1,
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predict_party_victory() {
        assert_eq!(
            "Radiant".to_string(),
            Solution::predict_party_victory("RD".to_string())
        );
        assert_eq!(
            "Dire".to_string(),
            Solution::predict_party_victory("RDD".to_string())
        );
        assert_eq!(
            "Radiant".to_string(),
            Solution::predict_party_victory("RRRRDD".to_string())
        );
        assert_eq!(
            "Dire".to_string(),
            Solution::predict_party_victory("DRRD".to_string())
        );
        assert_eq!(
            "Radiant".to_string(),
            Solution::predict_party_victory("DRRDRDRDRDDRDRDR".to_string())
        );
        assert_eq!(
            "Dire".to_string(),
            Solution::predict_party_victory("DDRRR".to_string())
        );
        assert_eq!(
            "Radiant".to_string(),
            Solution::predict_party_victory("RDRDRDDRDRDRDRDRRDRDRDRDRDRDDDDRRDRDRDRDRDRDRDRRRRRDRDRDRDRDDDDDRDRDRDRDRDRDRDRRDRDRDRDRDRDRRDRDRDRDRDRDRDRDRRDRDRDRDRDRRD".to_string())
        );
    }
}
