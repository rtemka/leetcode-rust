// https://leetcode.com/problems/number-of-digit-one/description/
struct Solution;

struct Count {
    fact: usize,
    max: usize,
    power: usize,
}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut p = 0;
        let mut i = 1;
        while i < n {
            p += 1;
            i *= 10;
        }
        let (prev_fact, prev_max, prev_power, prev_digit) =
            Self::count_digit_one_rec(n as usize, p);
        dbg!(prev_fact, prev_max, prev_power, prev_digit,);
        // let n = n as usize;

        // let mut digits: Vec<usize> = vec![1];
        // let mut count = 1;
        // let mut i = 10;
        // while i * 10 <= n {
        //     count = 9 * digits[i - 1] + 10_usize.pow(digits.len() as u32) + digits[i - 1];
        //     digits.push(count);
        //     i *= 10;
        // }
        // if i < n {
        //     count += Self::count_digit_one_rec(n - i);
        // }
        // 1991 - 1000 = 991 - 100 = 891
        // 1991 => 1000 + 900 + 90 + 1
        // 1115 => 1000 + 100 + 10 + 5
        //
        // 90 / 10 * n_prev + if (90 / 10)
        // 115 => 100 + 10 + 5
        prev_fact as i32
        // 301 + 21 + 2 + 1
    }

    pub fn count_digit_one_rec(n: usize, p: u32) -> (usize, usize, u32, usize) {
        let current = n % 10usize.pow(p);
        dbg!(p, n % 10usize.pow(p));
        println!();
        match n % 10usize.pow(p) {
            n @ 0 => (0, 1, 0, n),
            n @ 1..10 => (1, 1, 0, n),
            n => {
                let (prev_fact, prev_max, prev_power, prev_digit) =
                    Self::count_digit_one_rec(n, p - 1);
                let power = 10_usize.pow(prev_power + 1);
                // let digit = n / power;
                // max 1's on current power of 10.
                // max = 9 * N(1's on prev level) + 10^n + N(1's on prev level).
                let max = 9 * prev_max + power + prev_max;
                let digit = n / power;
                // let fact = n / power * prev_max + ((n / power > 1) as usize * power + prev_count);

                let fact = prev_fact
                    + if digit == 1 {
                        println!("WE HERE!");
                        // prev_max + (prev_digit * 10usize.pow(prev_power) + prev_max)
                        if prev_power == 0 {
                            prev_max + (prev_digit * 10usize.pow(prev_power) + prev_max)
                        } else {
                            prev_max + prev_fact * 2
                        }
                        // prev_max + (prev_digit * 10usize.pow(prev_power) + prev_max) // power
                        // prev_max + power // + (prev_max / digit) // + (n % power == 1) as usize
                    } else {
                        // (n / power - 1) * prev_max + power + prev_max // + (n % power == 1) as usize
                        digit * prev_max + power + prev_max // + (n % power == 1) as usize
                    };
                dbg!(
                    current, digit, power, max, fact, prev_fact, prev_max, prev_power, prev_digit,
                );
                println!();
                (fact, max, prev_power + 1, digit)
            }
        }
    }

    fn count_by_brute_force(n: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            count += i.to_string().chars().filter(|&c| c == '1').count();
        }
        count as i32
    }

    fn count_by_formula(powers_of_ten: usize) -> usize {
        if powers_of_ten == 0 {
            return 1;
        }
        let mut p: Vec<usize> = vec![1];
        let mut count = 1;
        for i in 1..=powers_of_ten {
            let c = 9 * p[i - 1] + 10_usize.pow(i as u32) + p[i - 1];
            dbg!(i, c, 10_usize.pow(i as u32));
            p.push(c);
            count = c;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_digit_one() {
        assert_eq!(46, Solution::count_by_brute_force(15));
        assert_eq!(46, Solution::count_by_brute_force(125));
        assert_eq!(5580, Solution::count_digit_one(11115));
        assert_eq!(463, Solution::count_digit_one(1115));
        assert_eq!(46, Solution::count_digit_one(115));
        assert_eq!(6, Solution::count_digit_one(13));
        assert_eq!(0, Solution::count_digit_one(0));
    }

    #[test]
    fn count_digit_one_brute_force() {
        assert_eq!(44, Solution::count_by_brute_force(115));
        assert_eq!(46, Solution::count_by_brute_force(15));
        assert_eq!(6, Solution::count_by_brute_force(13));
        assert_eq!(300, Solution::count_by_brute_force(999));
        assert_eq!(4000, Solution::count_by_brute_force(9999));
        assert_eq!(50000, Solution::count_by_brute_force(99999));
    }

    #[test]
    fn count_digit_one_by_formula() {
        assert_eq!(300, Solution::count_by_formula(2));
        assert_eq!(4000, Solution::count_by_formula(3));
        assert_eq!(50000, Solution::count_by_formula(4));
    }
}
