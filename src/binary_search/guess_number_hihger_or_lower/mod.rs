struct Solution;

impl Solution {
    unsafe fn guess_number<F>(n: i32, guess: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        let mut lo = 1;
        let mut hi = n as i64;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            println!("mid={mid}\tlo={lo}\thi={hi}");
            match guess(mid as i32) {
                0 => return mid as i32,
                -1 => hi = mid - 1,
                1 => lo = mid + 1,
                _ => (),
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_number() {
        unsafe {
            assert_eq!(
                6,
                Solution::guess_number(10, |x| {
                    if x < 6 {
                        1
                    } else if x > 6 {
                        -1
                    } else {
                        0
                    }
                })
            );
        }
        unsafe {
            assert_eq!(
                1,
                Solution::guess_number(1, |x| {
                    if x < 1 {
                        1
                    } else if x > 1 {
                        -1
                    } else {
                        0
                    }
                })
            );
        }
        unsafe {
            assert_eq!(
                1,
                Solution::guess_number(2, |x| {
                    if x < 1 {
                        1
                    } else if x > 1 {
                        -1
                    } else {
                        0
                    }
                })
            );
        }
        unsafe {
            assert_eq!(
                1702766719,
                Solution::guess_number(2126753390, |x| {
                    if x < 1702766719 {
                        1
                    } else if x > 1702766719 {
                        -1
                    } else {
                        0
                    }
                })
            );
        }
    }
}
