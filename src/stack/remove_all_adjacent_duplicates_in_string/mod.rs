// #1047
// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/description/
struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::<char>::new();
        for c in s.chars() {
            // If the current character is the same as the top character on the stack,
            // a pair of adjacent duplicates has been formed. So, pop the top character
            // from the stack.
            if let Some(_) = stack.last().filter(|&&top| c == top) {
                stack.pop();
            } else {
                // Otherwise, push the current character onto the stack.
                stack.push(c);
            }
        }
        // Return the remaining characters as a String.
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_all_adjacent_duplicates_in_string() {
        assert_eq!(
            "ca".to_string(),
            Solution::remove_duplicates("abbaca".to_string())
        )
    }
}
