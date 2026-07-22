// #3753
// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i
// See also: #3751
// Explanation:
// https://codeforces.com/blog/entry/53960
// https://github.com/flash7even/Programming-Contest/blob/master/Code%20Repository/Dynamic%20Programming/Digit%20DP/Digit-DP.cpp
struct Solution;

/// `TABLE[pos][tight][count][prev][dir]` = Number of valid numbers `<=` b from this state
/// `pos`   = current position from left side (zero based) (0..9 and 10 for "empty")
/// `tight` = the number we are building has already become smaller than b? **`0 = no, 1 = yes`**
/// `count` = number of times we have seen zigzag.
/// `prev`  = pos-1 number.
/// `dir` = where do we came from to the prev digit **`0 = from above, 1 = from the bottom, 2 = not
/// determined yet / plato`**.
type Table = [[[[[i64; 3]; 11]; 20]; 2]; 20];

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        // Constraints:
        //
        // 1 <= num1 <= num2 <= 10^5

        // Инициализируем таблицу DP
        let mut dp: Table = [[[[[-1_i64; 3]; 11]; 20]; 2]; 20];

        // Решаем сначала для [0..num1] и вычитаем из этого [0..num1-1].
        Self::solve(&mut dp, num2 as usize) - Self::solve(&mut dp, (num1 - 1) as usize)
    }

    fn solve(dp: &mut Table, mut num: usize) -> i64 {
        // Превращаем число в последовательность цифр 324 => [3, 2, 4]
        let mut seq = Vec::new();
        while num > 0 {
            seq.push(num % 10);
            num /= 10;
        }

        seq.reverse();

        // Чистим таблицу DP
        *dp = [[[[[-1_i64; 3]; 11]; 20]; 2]; 20];

        // 10 является sentinel value чтобы понимать что у нас идут ведущие нули и число еще не началось.
        // Стартуем: prev = 10 (пусто), dir = 2 (направление не задано)
        Self::call(&seq, dp, 0, 0, 0, 10, 2)
    }

    fn call(
        num: &[usize],
        dp: &mut Table,
        pos: usize,
        tight: usize,
        count: usize,
        prev: usize,
        dir: usize,
    ) -> i64 {
        // Базовый случай: мы дошли до конца числа
        if pos == num.len() {
            return count as i64;
        }

        // Мемоизация
        if dp[pos][tight][count][prev][dir] != -1 {
            return dp[pos][tight][count][prev][dir];
        }

        let limit = if tight == 0 { num[pos] } else { 9 };
        let mut res = 0;

        // Перебираем ВСЕ возможные цифры, так как любая цифра валидна,
        // просто не каждая дает +1 к зигзагам.
        for digit in 0..=limit {
            let new_tight = if tight == 0 && digit < limit {
                1
            } else {
                tight
            };

            if prev == 10 {
                // Число еще не началось (ведущие нули)
                if digit == 0 {
                    res += Self::call(num, dp, pos + 1, new_tight, count, 10, 2);
                } else {
                    // Поставили первую цифру. Направление пока неизвестно.
                    res += Self::call(num, dp, pos + 1, new_tight, count, digit, 2);
                }
                continue;
            }

            if dir == 2 {
                // У нас есть ровно одна предыдущая цифра (или было плато).
                // Зигзага здесь быть не может, мы просто задаем направление.
                res += Self::call(
                    num,
                    dp,
                    pos + 1,
                    new_tight,
                    count,
                    digit,
                    if digit > prev {
                        1
                    } else if digit < prev {
                        0
                    } else {
                        2
                    },
                );
                continue;
            }

            // У нас есть четкое направление (0 = вниз, 1 = вверх).
            if digit == prev {
                // Если цифры равны, значит ПЛАТО, направление становится неопределенным.
                res += Self::call(num, dp, pos + 1, new_tight, count, digit, 2);
            } else if dir == 1 {
                // Шли ВВЕРХ. Сейчас на ПИКЕ.
                if digit < prev {
                    // Мы пошли вниз. Зигзаг (ПИК) состоялся.
                    res += Self::call(num, dp, pos + 1, new_tight, count + 1, digit, 0); // Теперь мы идем вниз
                } else {
                    // Мы продолжили идти вверх. Зигзага нет.
                    res += Self::call(num, dp, pos + 1, new_tight, count, digit, 1);
                }
            } else {
                // dir == 0: Шли ВНИЗ. Сейчас во ВПАДИНЕ.
                if digit > prev {
                    // Мы пошли вверх. Зигзаг (ВПАДИНА) состоялся.
                    res += Self::call(num, dp, pos + 1, new_tight, count + 1, digit, 1); // Теперь мы идем вверх
                } else {
                    // Мы продолжили идти вниз. Зигзага нет.
                    res += Self::call(num, dp, pos + 1, new_tight, count, digit, 0);
                }
            }
        }

        // Сохраняем результат для мемоизации.
        dp[pos][tight][count][prev][dir] = res;

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_waviness_of_numbers_in_range_2() {
        assert_eq!(3, Solution::total_waviness(120, 130));
        assert_eq!(3, Solution::total_waviness(198, 202));
        assert_eq!(2, Solution::total_waviness(4848, 4848));
    }
}
