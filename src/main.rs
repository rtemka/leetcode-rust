#![allow(dead_code)]

// Important and usefull links.
// https://leetcode.com/discuss/post/665604/important-and-useful-links-from-all-over-ocy8/

use rand::Rng;
mod backtracking;
mod binary_search;
mod bit_manipulation;
mod dynamic_programming;
mod fast_and_slow_pointers;
mod graph;
mod greedy;
mod hash_table;
mod heap;
mod intervals;
mod linked_list;
mod math_and_geometry;
mod misc;
mod prefix_sum;
mod sliding_window;
mod sorting_and_searching;
mod stack;
mod tree;
mod trie;
mod two_pointers;

fn main() {
    let tags = vec![
        "two pointers",
        "hash table",
        "linked list",
        "fast and slow pointers",
        "sliding window",
        "binary search",
        "stack",
        "heap",
        "intervals",
        "prefix sum",
        "tree",
        "trie",
        "graph",
        "backtracking",
        "dynamic programming",
        "greedy",
        "sorting and searching",
        "bit manipulation",
        "math and geometry",
        "misc",
    ];
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..tags.len());
    println!("============================================");
    println!("\n\n And today subject is: {}!\n\n", tags[i]);
    println!("============================================");
}
