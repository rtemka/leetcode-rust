// Constraints:
//    0 <= key <= 10^6
//    At most 10^4 calls will be made to add, remove, and contains.

use std::ops::BitXor;

/// N is a number of items
/// in the hashset bucket.
const BN: usize = 10;
const TN: usize = 2500;
const K: usize = 0x517cc1b727220a95;

// https://leetcode.com/problems/design-hashset/description/
/// The hashing function is from
/// [this article](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html)
/// and from the FxHasher of the rust compiler.
struct MyHashSet {
    table: Vec<[Option<i32>; BN]>,
}

impl MyHashSet {
    #[inline]
    fn new() -> Self {
        Self {
            table: vec![[None; BN]; TN],
        }
    }

    #[inline]
    fn add(&mut self, key: i32) {
        let idx = Self::hash(key) % self.table.len();
        println!("hashed index for add:{}", idx);
        let mut free_slot = BN + 1;
        for i in 0..BN {
            match self.table[idx][i] {
                Some(v) if v == key => return,
                None => {
                    if free_slot == BN + 1 {
                        free_slot = i;
                    }
                }
                _ => continue,
            }
        }
        if free_slot <= BN {
            println!("add in self.table[{idx}][{free_slot}]");
            self.table[idx][free_slot] = Some(key)
        } else {
            panic!("no free positons in bucket");
        }
    }

    #[inline]
    fn hash(key: i32) -> usize {
        let i = if key < 0 {
            (-key as usize).wrapping_mul(2)
        } else {
            key as usize
        };
        0_usize.rotate_left(5).bitxor(i).wrapping_mul(K)
    }

    #[inline]
    fn remove(&mut self, key: i32) {
        let idx = Self::hash(key) % self.table.len();
        println!("hashed index for remove:{}", idx);
        for i in 0..BN {
            if let Some(v) = self.table[idx][i] {
                if v == key {
                    self.table[idx][i] = None;
                    return;
                }
            }
        }
    }

    #[inline]
    fn contains(&self, key: i32) -> bool {
        let idx = Self::hash(key) % self.table.len();
        println!("hashed index for contains:{}", idx);
        for i in 0..BN {
            if let Some(v) = self.table[idx][i] {
                if v == key {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_hashset() {
        let mut set = MyHashSet::new();
        for i in 0..10000 {
            println!("it is {i}-th item");
            set.add(i);
        }
        for i in 100 / 2..100 {
            set.remove(i);
        }
        for i in 0..100 / 2 {
            assert!(set.contains(i));
        }
    }
}
