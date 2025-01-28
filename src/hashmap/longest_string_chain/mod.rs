// https://leetcode.com/problems/longest-string-chain/description/
struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    // initial - 84/86 passed
    // not works cause one word can diverge on multiple search paths
    pub fn longest_str_chain_initial(words: Vec<String>) -> i32 {
        let set: HashSet<&String> = words.iter().collect();
        let mut longest = 0;
        let mut workstr = String::with_capacity(16); // constraints: words[i].len <= 16;
        for word in &words {
            let mut cur_longest = 1; // cause this word counts too
            let mut cur_word = word;
            'outer: loop {
                workstr.clear();
                for i in 0..cur_word.len() {
                    workstr.push_str(&cur_word[..i]);
                    workstr.push_str(&cur_word[i + 1..]);
                    if let Some(&w) = set.get(&workstr) {
                        cur_longest += 1;
                        cur_word = w;
                        continue 'outer;
                    }
                    workstr.clear();
                }
                break;
            }
            longest = longest.max(cur_longest);
        }
        longest
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        // remember all words.
        let set: HashSet<&String> = words.iter().collect();
        // have to use map for memoization because otherwise will get TLE.
        let mut map: HashMap<&String, i32> = HashMap::with_capacity(words.len());
        let mut longest = 1;
        // this is our scrathpad string to reuse same memory.
        // constraints: words[i].len <= 16;
        let mut workstr = String::with_capacity(16);
        for word in &words {
            // if this word is smaller than max
            // then chain never be larger
            if word.len() < longest as usize {
                continue;
            }
            let _ = Self::longest_str_chain_rec(
                &mut workstr,
                &set,
                &mut map,
                word,
                1, /*start from this word*/
                &mut longest,
            );
        }
        longest
    }

    fn longest_str_chain_rec<'a>(
        workstr: &mut String,
        set: &HashSet<&'a String>,
        map: &mut HashMap<&'a String, i32>,
        word: &'a String,
        cur_chain: i32,
        longest: &mut i32,
    ) -> i32 {
        // if we already computed the chain from this word downward.
        if let Some(&v) = map.get(word) {
            // println!("MEMO {:?}\t{}", word, v);
            return v + cur_chain;
        }
        // we count longest chain for this particular word
        // and also the chain length from this word downward.
        let mut cur_longest = cur_chain;
        workstr.clear();
        for i in 0..word.len() {
            // exclude one letter from word and look it up in the set.
            workstr.push_str(&word[..i]);
            workstr.push_str(&word[i + 1..]);
            // println!("cur={:?}\tcheck word: is {:?}", word, workstr);
            if let Some(&word) = set.get(workstr) {
                // follow the path recursively
                cur_longest = cur_longest.max(Self::longest_str_chain_rec(
                    workstr,
                    set,
                    map,
                    word,
                    cur_chain + 1,
                    longest,
                ));
            }
            workstr.clear();
        }
        // println!("COMPLETED for {:?}\t{}", word, cur_longest);

        // remember the chain length from this word downward. 
        map.insert(word, cur_longest - cur_chain);
        // compute longest chain overall.
        *longest = i32::max(*longest, cur_longest);
        cur_longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_str_chain() {
        assert_eq!(
            7,
            Solution::longest_str_chain(vec![
                "ksqvsyq".to_string(),
                "ks".to_string(),
                "kss".to_string(),
                "czvh".to_string(),
                "zczpzvdhx".to_string(),
                "zczpzvh".to_string(),
                "zczpzvhx".to_string(),
                "zcpzvh".to_string(),
                "zczvh".to_string(),
                "gr".to_string(),
                "grukmj".to_string(),
                "ksqvsq".to_string(),
                "gruj".to_string(),
                "kssq".to_string(),
                "ksqsq".to_string(),
                "grukkmj".to_string(),
                "grukj".to_string(),
                "zczpzfvdhx".to_string(),
                "gru".to_string(),
            ])
        );

        assert_eq!(
            4,
            Solution::longest_str_chain(vec![
                "a".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "bd".to_string(),
                "abc".to_string(),
                "abd".to_string(),
                "abdd".to_string(),
            ])
        );

        assert_eq!(
            4,
            Solution::longest_str_chain(vec![
                "a".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bca".to_string(),
                "bda".to_string(),
                "bdca".to_string(),
                "bdca".to_string(),
            ])
        );
        assert_eq!(
            5,
            Solution::longest_str_chain(vec![
                "xbc".to_string(),
                "pcxbcf".to_string(),
                "xb".to_string(),
                "cxbc".to_string(),
                "pcxbc".to_string(),
            ])
        );

        assert_eq!(1, Solution::longest_str_chain(vec!["x".to_string()]));

        assert_eq!(
            1,
            Solution::longest_str_chain(vec!["abcd".to_string(), "dbqca".to_string(),])
        );

        assert_eq!(
            3,
            Solution::longest_str_chain(vec![
                "zczpzvdhx".to_string(),
                "zczpzvh".to_string(),
                "zczpzvhx".to_string()
            ])
        );
    }
}
