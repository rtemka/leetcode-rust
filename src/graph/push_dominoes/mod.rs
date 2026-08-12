// #838
// https://leetcode.com/problems/push-dominoes/description/

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        unsafe {
            let dominoes = dominoes.as_bytes_mut();

            let mut queue: VecDeque<usize> = dominoes
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| if c != b'.' { Some(i) } else { None })
                .collect();

            let mut visited = vec![0; dominoes.len()];

            while let Some(i) = queue.pop_front() {
                let second = visited[i] + 1;

                match dominoes[i] {
                    b'R' if i + 1 < dominoes.len() => {
                        let right = i + 1;
                        if dominoes[right] == b'L' && visited[right] == second {
                            dominoes[right] = b'.';
                        } else if dominoes[right] == b'.' {
                            dominoes[right] = b'R';
                            visited[right] = second;
                            queue.push_back(right);
                        }
                    }
                    b'L' if i > 0 => {
                        let left = i - 1;
                        if dominoes[left] == b'R' && visited[left] == second {
                            dominoes[left] = b'.';
                        } else if dominoes[left] == b'.' {
                            dominoes[left] = b'L';
                            visited[left] = second;
                            queue.push_back(left);
                        }
                    }
                    _ => (),
                }
            }
        }
        dominoes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_dominoes() {
        assert_eq!("RR.L", Solution::push_dominoes("RR.L".to_string()));

        assert_eq!(
            "LL.RR.LLRRLL..",
            Solution::push_dominoes(".L.R...LR..L..".to_string())
        )
    }
}
