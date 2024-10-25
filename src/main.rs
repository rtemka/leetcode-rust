#![allow(dead_code)]
mod array;
mod array_and_string;
mod backtracking;
mod binary_search;
mod binary_tree;
mod bit_manipulation;
mod daily;
mod dp;
mod easy;
mod graph;
mod hashmap;
mod linked_list;
mod monotonic_stack;
mod queue;
mod recursion;

fn main() {
    println!(
        "{}",
        vec![1, 2] == easy::two_sum_1::two_sum_on(vec![3, 2, 4], 6)
    );
}
