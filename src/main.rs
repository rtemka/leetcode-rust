#![allow(dead_code)]
mod array;
mod daily;
mod easy;

fn main() {
    println!(
        "{}",
        vec![1, 2] == easy::two_sum_1::two_sum_on(vec![3, 2, 4], 6)
    );
}
