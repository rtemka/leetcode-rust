#![allow(dead_code)]

use rand::Rng;
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
    let tags = vec![
        "bit manipulation",
        "linked list",
        "array",
        "strings",
        "DP",
        "DFS",
        "BFS",
        "backtracking",
        "binary search",
        "prefix sum",
        "queue",
        "recursion",
        "binary tree",
        "stack",
        "monotonic stack",
        "bitmask",
        "tree",
        "two pointers",
        "hash table",
        "matrix",
        "sliding window",
        "trie",
        "heap(priority queue)",
        "greedy",
        "counting",
        "graph",
    ];
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..tags.len());
    println!("============================================");
    println!("\n\n And today subject is: {}!\n\n", tags[i]);
    println!("============================================");
}
