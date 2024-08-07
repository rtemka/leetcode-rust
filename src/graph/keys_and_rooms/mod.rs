use std::collections::HashSet;

// https://leetcode.com/problems/keys-and-rooms/description
struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::with_capacity(rooms.len());
        Self::visit(&rooms, 0, &mut visited);
        visited.len() == rooms.len()
    }

    fn visit(rooms: &Vec<Vec<i32>>, cur_room: usize, visited: &mut HashSet<usize>) {
        if !visited.insert(cur_room) {
            return;
        }
        for key in &rooms[cur_room] {
            Self::visit(rooms, *key as usize, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_visit_all_rooms() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![]
        ]));
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![]
        ]));
    }
}
