#![allow(dead_code)]
mod array;
mod array_and_string;
mod bit_manipulation;
mod daily;
mod easy;
mod hashmap;
mod linked_list;
mod recursion;

fn main() {
    println!(
        "{}",
        vec![1, 2] == easy::two_sum_1::two_sum_on(vec![3, 2, 4], 6)
    );
}
