// https://leetcode.com/problems/solving-questions-with-brainpower/description
struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut memo = vec![-1; questions.len()];
        Self::most_points_rec(&questions, &mut memo, 0)
    }

    fn most_points_rec(questions: &Vec<Vec<i32>>, memo: &mut Vec<i64>, i: usize) -> i64 {
        // If we pass the boundary.
        if i >= memo.len() {
            return 0;
        }
        // Check if already computed.
        if memo[i] > 0 {
            return memo[i];
        }
        // Destruct.
        let (points, brainpower) = (questions[i][0] as i64, questions[i][1] as usize);
        // We have 2 choices:
        // Either solve and skip next N questions.
        let solve = points as i64 + Self::most_points_rec(questions, memo, i + 1 + brainpower);
        // Or skip this and go to the next.
        let skip = Self::most_points_rec(questions, memo, i + 1);
        // Check which was better.
        let result = i64::max(solve, skip);
        memo[i] = result;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving_questions_with_brainpower() {
        assert_eq!(
            5,
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
        );

        assert_eq!(
            157,
            Solution::most_points(vec![
                vec![21, 5],
                vec![92, 3],
                vec![74, 2],
                vec![39, 4],
                vec![58, 2],
                vec![5, 5],
                vec![49, 4],
                vec![65, 3]
            ])
        );

        assert_eq!(
            7,
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ])
        );

        assert_eq!(8, Solution::most_points(vec![vec![8, 1]]));
    }
}
