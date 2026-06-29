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

static PATTERNS: LazyLock<[Pattern; 6]> = LazyLock::new(|| {
    [
        Pattern {
            name: "binary_search".to_string(),
            variations: vec![
                Variation {
                    name: "exact".to_string(),
                    description: "нужно получить точное совпадение".to_string(),
                    ..Default::default()
                },
                Variation {
                    name: "lower_bound".to_string(),
                    description: "нужно получить место, где было бы вставлено значение слева"
                        .to_string(),
                    ..Default::default()
                },
                Variation {
                    name: "upper_bound".to_string(),
                    description: "нужно получить место, где было бы вставлено значение справа"
                        .to_string(),
                    ..Default::default()
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
                    Some("Given an array nums of distinct integers, return all the possible permutations"
                        .to_string()),
            }],
        },
        Pattern {
            name: "two_pointers".to_string(),
            variations: vec![
                Variation {
                    name: "inward_traversal".to_string(),
                    description: "В этой стратегии указатели начинают с противоположных концов структуры данных и постепенно двигаются друг к другу"
                        .to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/two-sum-ii-input-array-is-sorted".to_string(),
                        description: None,
                    }),
                },
                Variation {
                    name: "unidirectional_traversal".to_string(),
                    description: "В этой стратегии перемещение обоих указателей начинается с одного конца структуры данных (обычно с начала) и происходит в одном направлении"
                        .to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/move-zeroes/description/".to_string(),
                        description: None,
                    }),
                },
                Variation {
                    name: "staged_traversal".to_string(),
                    description: "Здесь один указатель проходит по структуре данных, и, как только он встречает элемент".to_string() +
                        ", удовлетворяющий определенному условию, второй указатель начинает двигаться от этой позиции для дальнейшего анализа",
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/next-permutation".to_string(),
                        description: None,
                    }),
                },
            ],
            problems: vec![
                Problem{
                    url: "https://leetcode.com/problems/move-zeroes/description/".to_string(),
                    description: Some("unidirectional_traversal".to_string()),
                },
                Problem{
                    url: "https://leetcode.com/problems/3sum".to_string(),
                    description: Some("inward_traversal".to_string()),
                },
                Problem{
                    url: "https://leetcode.com/problems/container-with-most-water".to_string(),
                    description: Some("inward_traversal".to_string()),
                },
                Problem{
                    url: "https://leetcode.com/problems/next-permutation".to_string(),
                    description: Some("staged_traversal".to_string()),
                },
            ],
        },
        Pattern {
            name: "sliding_window".to_string(),
            variations: vec![
                Variation {
                    name: "fixed_window".to_string(),
                    description: "Фиксированное скользящее окно сохраняет постоянную длину, двигаясь по структуре данных"
                        .to_string(),
                    canonical_problem: Some(Problem{
                        url: "http://leetcode.com/problems/find-all-anagrams-in-a-string/description/".to_string(),
                        description: None,
                    }),
                },
                Variation {
                    name: "dynamic_window".to_string(),
                    description: "динамическое окнo может изменять свою длину — расширяться или сужаться — при перемещении по структуре данных"
                        .to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/longest-substring-without-repeating-characters/".to_string(),
                        description: None,
                    }),
                },
            ],
            problems: vec![],
            // problems: vec![
            //     Problem{
            //         url: "http://leetcode.com/problems/find-all-anagrams-in-a-string/description/".to_string(),
            //         description: Some("fixed_window".to_string()),
            //     },
            //     Problem{
            //         url: "https://leetcode.com/problems/longest-substring-without-repeating-characters/".to_string(),
            //         description: Some("dynamic_window".to_string()),
            //     },
            //
            // ],
        },
        Pattern {
            name: "graph_traversal".to_string(),
            variations: vec![
                Variation {
                    name: "dfs_recursive".to_string(),
                    description: "реализуй поиск в глубину с помощью рекурсии".to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/maximum-depth-of-binary-tree".to_string(),
                        description: None,
                    }),
                },
                Variation {
                    name: "dfs_iterative".to_string(),
                    description: "реализуй поиск в глубину итеративно с помощью стэка".to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/maximum-depth-of-binary-tree".to_string(),
                        description: Some("Найдите максимальную глубину бинарного дерева".to_string()),
                    }),
                },
                Variation {
                    name: "bfs".to_string(),
                    description: "реализуй поиск в ширину".to_string(),
                    canonical_problem: Some(Problem{
                        url: "https://leetcode.com/problems/binary-tree-level-order-traversal".to_string(),
                        description: None,
                    }),
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
                    ..Default::default()
                },
                Variation {
                    name: "bubble_sort".to_string(),
                    description: "реализуй сортировку пузырьком".to_string(),
                    ..Default::default()
                },
                Variation {
                    name: "insertion_sort".to_string(),
                    description: "реализуй сортировку вставкой".to_string(),
                    ..Default::default()
                },
                Variation {
                    name: "selection_sort".to_string(),
                    description: "реализуй сортировку выбором".to_string(),
                    ..Default::default()
                },
                Variation {
                    name: "quicksort".to_string(),
                    description: "реализуй быструю сортировку".to_string(),
                    ..Default::default()
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
            if let Some(cp) = &v.canonical_problem {
                println!(
                    "\t Variation canonical problem: url: {:?}, description: {:?}",
                    cp.url, cp.description
                );
            }
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct Problem {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Variation {
    pub name: String,
    pub description: String,
    pub canonical_problem: Option<Problem>,
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
