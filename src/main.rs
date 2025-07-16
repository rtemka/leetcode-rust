#![allow(dead_code)]

// Important and usefull links.
// https://leetcode.com/discuss/post/665604/important-and-useful-links-from-all-over-ocy8/

use rand::Rng;
mod array;
mod array_and_string;
mod backtracking;
mod bfs_dfs;
mod binary_search;
mod binary_tree;
mod bit_manipulation;
mod daily;
mod dp;
mod easy;
mod graph;
mod greedy;
mod hashmap;
mod linked_list;
mod math;
mod stack;
mod prefix_sum;
mod priority_queue;
mod queue;
mod recursion;
mod sliding_window;
mod two_pointers;

fn main() {
    let tags = vec![
        "bit manipulation",
        "sorting(I mean, the handwritten!)",
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
