struct Solution;

impl Solution {
    pub fn find_array(mut pref: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        let mut i = 0;
        while i < pref.len() {
            pref[i] = xor ^ pref[i];
            xor ^= pref[i];
            i += 1;
        }
        pref
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_array_of_prefix_xor() {
        assert_eq!(vec![13], Solution::find_array(vec![13]));

        assert_eq!(
            vec![5, 7, 2, 3, 2],
            Solution::find_array(vec![5, 2, 0, 3, 1])
        );
    }
}
