// https://leetcode.com/problems/ugly-number-ii/description
struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut v = Vec::with_capacity(n);
        v.push(1);
        let mut i = 1;
        let (mut m2, mut m3, mut m5) = (0, 0, 0);
        while i < n {
            let two_multiple = v[m2] * 2;
            let three_multiple = v[m3] * 3;
            let five_multiple = v[m5] * 5;

            v.push(i32::min(
                two_multiple,
                i32::min(three_multiple, five_multiple),
            ));

            m2 = if v[i] == two_multiple { m2 + 1 } else { m2 };
            m3 = if v[i] == three_multiple { m3 + 1 } else { m3 };
            m5 = if v[i] == five_multiple { m5 + 1 } else { m5 };

            i += 1;
        }
        v[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_ugly_number() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}
