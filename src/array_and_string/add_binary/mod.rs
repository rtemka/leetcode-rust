// https://leetcode.com/problems/add-binary/description/
struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut v: Vec<char> = Vec::new();
        let mut carry = false;
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let (mut ai, mut bi) = (a_bytes.len() as i32, b_bytes.len() as i32);
        while ai > 0 || bi > 0 {
            ai -= 1;
            bi -= 1;
            let av = if ai < 0 { 0x30 } else { a_bytes[ai as usize] };
            let bv = if bi < 0 { 0x30 } else { b_bytes[bi as usize] };
            v.push(match (av, bv) {
                (0x31, 0x31) => {
                    if carry {
                        '1'
                    } else {
                        carry = true;
                        '0'
                    }
                }
                (0x30, 0x30) => {
                    if carry {
                        carry = false;
                        '1'
                    } else {
                        '0'
                    }
                }
                (_, _) => {
                    if carry {
                        '0'
                    } else {
                        '1'
                    }
                }
            });
            println!("a:{}|b{} --> {:?}", a, b, v);
        }
        if carry {
            v.push('1');
        }
        v.into_iter().rev().collect()
    }
    // 1010
    // 1011
    //10101
    //
    // 1111
    // 1111
    //    0
    // 101
    //  11
    // 1000

    // Leetcode has protection against this hack))
    // the positive overflow will occur.
    pub fn add_binary_cheat(a: String, b: String) -> String {
        let a = u64::from_str_radix(&a, 2).unwrap();
        let b = u64::from_str_radix(&b, 2).unwrap();
        format!("{:b}", a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary() {
        assert_eq!(
            "100",
            Solution::add_binary_cheat("11".to_owned(), "1".to_owned())
        );
        assert_eq!(
            "10101",
            Solution::add_binary_cheat("1010".to_owned(), "1011".to_owned())
        );
        assert_eq!("100", Solution::add_binary("11".to_owned(), "1".to_owned()));
        assert_eq!(
            "10101",
            Solution::add_binary("1010".to_owned(), "1011".to_owned())
        );
    }
}
