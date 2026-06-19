#![allow(dead_code)]
#![feature(variant_count)]

// Important and usefull links.
// https://leetcode.com/discuss/post/665604/important-and-useful-links-from-all-over-ocy8/

use rand::{RngExt, seq::index};
use std::sync::LazyLock;

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

static PATTERNS: LazyLock<[Pattern; 4]> = LazyLock::new(|| {
    [
        Pattern {
            name: "binary_search".to_string(),
            variations: vec![
                Variation {
                    name: "exact".to_string(),
                    description: "нужно получить точное совпадение".to_string(),
                },
                Variation {
                    name: "lower_bound".to_string(),
                    description: "нужно получить место, где было бы вставлено значение слева"
                        .to_string(),
                },
                Variation {
                    name: "upper_bound".to_string(),
                    description: "нужно получить место, где было бы вставлено значение справа"
                        .to_string(),
                },
            ],
            problems: vec![],
        },
        Pattern {
            name: "backtracking".to_string(),
            variations: vec![],
            problems: vec![Problem {
                url: "https://leetcode.com/problems/permutations".to_string(),
                description:
                    "Given an array nums of distinct integers, return all the possible permutations"
                        .to_string(),
            }],
        },
        Pattern {
            name: "sliding_window".to_string(),
            variations: vec![
                Variation {
                    name: "extend_and_shrink".to_string(),
                    description: "сначала расширяем чтобы окно стало валидным, потом сужаем пока не станет невалидным"
                        .to_string(),
                },
                Variation {
                    name: "extend_and_slide".to_string(),
                    description: "сначала расширяем чтобы окно стало валидным, потом двигаем окно по массиву"
                        .to_string(),
                },
            ],
            problems: vec![],
        },
        Pattern {
            name: "sorting".to_string(),
            variations: vec![
                Variation {
                    name: "merge_sort".to_string(),
                    description: "реализуй сортировку слиянием".to_string(),
                },
                Variation {
                    name: "bubble_sort".to_string(),
                    description: "реализуй сортировку пузырьком".to_string(),
                },
            ],
            problems: vec![],
        },
    ]
});

static SUBJECTS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
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
    ]
});

static LANGUAGES: [Language; 3] = [Language::Python, Language::Golang, Language::Rust];

fn main() {
    let daily_routine = DailyRepetition::new(4);

    println!("============================================");
    println!("\n\n Patterns for daily repetition today:\n");
    for p in daily_routine.patterns_to_repeat {
        println!("\t Pattern: {:?}", p.name);
        for v in p.variations {
            println!(
                "\t Variation: name: {:?}, description: {:?}",
                v.name, v.description
            );
        }
        for problem in p.problems {
            println!(
                "\t Problem: url: {:?} description: {:?}",
                problem.url, problem.description
            );
        }
        println!();
    }
    println!("\n Language: {:?}\n", daily_routine.language);
    println!("============================================");

    println!(" Additionaly");
    println!(
        "\n\n Try solve the problem on leetcode.com with subject: {:?}!\n\n",
        daily_routine.daily_subject
    );
    println!("============================================");
}

#[derive(Debug, Clone)]
pub struct DailyRepetition<'a> {
    // This is patterns in your daily repetition routine. You must implement them from scratch.
    pub patterns_to_repeat: Vec<ChoosenPattern<'a>>,

    pub language: Language,

    // Means you should go to leetcode and solve the problem in given subject.
    pub daily_subject: &'static str,
}

impl<'a> DailyRepetition<'a> {
    pub fn new(n_patterns: usize) -> Self {
        let indicies =
            generate_unique_indices(PATTERNS.len(), usize::min(n_patterns, PATTERNS.len()));

        let mut choosen_patterns = Vec::with_capacity(n_patterns);
        for i in indicies {
            choosen_patterns.push(ChoosenPattern::new(&PATTERNS[i], 1, 1));
        }

        let lang = rand::rng().random_range(0..LANGUAGES.len());

        Self {
            patterns_to_repeat: choosen_patterns,
            daily_subject: Self::choose_subject(),
            language: LANGUAGES[lang].clone(),
        }
    }

    fn choose_subject() -> &'static str {
        let mut rng = rand::rng();
        let i = rng.random_range(0..SUBJECTS.len());
        SUBJECTS[i]
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub variations: Vec<Variation>,
    pub problems: Vec<Problem>,
}

#[derive(Debug, Clone)]
pub struct ChoosenPattern<'a> {
    pub name: &'a str,
    pub variations: Vec<&'a Variation>,
    pub problems: Vec<&'a Problem>,
}

impl<'a> ChoosenPattern<'a> {
    pub fn new(p: &'a Pattern, num_variations: usize, num_problems: usize) -> Self {
        let indicies = generate_unique_indices(
            p.variations.len(),
            usize::min(num_variations, p.variations.len()),
        );
        let variations = indicies.into_iter().map(|i| &p.variations[i]).collect();

        let indicies =
            generate_unique_indices(p.problems.len(), usize::min(num_problems, p.problems.len()));
        let problems = indicies.into_iter().map(|i| &p.problems[i]).collect();

        Self {
            name: &p.name,
            variations,
            problems,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Variation {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    Golang,
    Python,
}

impl Language {
    pub fn count() -> usize {
        std::mem::variant_count::<Self>()
    }
}

fn generate_unique_indices(upper_bound: usize, n: usize) -> Vec<usize> {
    assert!(
        n <= upper_bound,
        "Requested more unique indices than available in the range!"
    );

    let mut rng = rand::rng();

    // Generates N unique indices from 0..range_size
    index::sample(&mut rng, upper_bound, n).into_vec()
}
