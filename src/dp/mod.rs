// Learning DP.
// https://leetcode.com/discuss/post/475924/my-experience-and-notes-for-learning-dp-ynooz/
// Patterns
// https://leetcode.com/discuss/post/458695/dynamic-programming-patterns-by-aatalyk-pmgr/
// Great explanation of DP problems.
// https://leetcode.com/problems/house-robber/solutions/156523/From-good-to-great.-How-to-approach-most-of-DP-problems/
// Problems to practice.
// https://leetcode.com/discuss/post/662866/dp-for-beginners-problems-patterns-sampl-atdb/

pub mod delete_operation_for_two_strings;
pub mod edit_distance;
pub mod frog_jump;
pub mod house_robber;
pub mod jump_game;
pub mod jump_game_2;
pub mod longest_common_subsequence;
pub mod longest_palindromic_subsequence;
pub mod maximum_subarray;
pub mod min_cost_climbing_stairs;
pub mod minimum_path_sum;
pub mod nth_tribonacci_number;
pub mod solving_questions_with_brainpower;
pub mod ugly_number_2;
pub mod unique_paths;

pub fn print_table<T: std::fmt::Debug>(v: &[Vec<T>]) {
    for line in v {
        println!("{:?}", line);
    }
}
