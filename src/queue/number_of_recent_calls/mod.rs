use std::collections::VecDeque;

// https://leetcode.com/problems/number-of-recent-calls/description
struct RecentCounter {
    deq: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            deq: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.deq.push_back(t);
        loop {
            match self.deq.front() {
                Some(&elem) => {
                    if elem < t - 3000 {
                        self.deq.pop_front();
                    } else {
                        break;
                    }
                }
                None => break,
            };
        }
        self.deq.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_call_counter() {
        let mut call_counter = RecentCounter::new();
        assert_eq!(1, call_counter.ping(1));
        assert_eq!(2, call_counter.ping(100));
        assert_eq!(3, call_counter.ping(3001));
        assert_eq!(3, call_counter.ping(3002));
        assert_eq!(1, call_counter.ping(100000));
    }
}
