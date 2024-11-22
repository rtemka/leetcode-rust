struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in image.iter_mut() {
            let (mut lo, mut hi) = (0, row.len() - 1);
            while lo <= hi {
                if lo == hi {
                    row[lo] ^= 1;
                    break;
                }
                row[lo] ^= 1;
                row[hi] ^= 1;
                row.swap(lo, hi);
                (lo, hi) = (lo + 1, hi - 1);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_and_convert_image() {
        assert_eq!(
            vec![vec![0]],
            Solution::flip_and_invert_image(vec![vec![1]])
        );
        assert_eq!(
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]])
        );
    }
}
