use std::collections::HashMap;

//https://leetcode.com/problems/longest-palindrome/description/
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::with_capacity(s.len());
        for c in s.chars().into_iter() {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        // Constraints:
        // 1 <= s.length <= 2000
        let plus_one = map.iter().any(|(_, count)| *count == 1 || *count & 1 == 1);
        map.into_iter().filter(|(_, count)| *count > 1).fold(
            0 + plus_one as i32,
            |acc, (_, count)| {
                acc + (count - (count & 1 == 1) as i32) // make even, we only
                                                        // conserned with even amount of chars
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome() {
        assert_eq!(3, Solution::longest_palindrome("ccc".to_string()));
        assert_eq!(2, Solution::longest_palindrome("bb".to_string()));
        assert_eq!(1, Solution::longest_palindrome("abc".to_string()));
        assert_eq!(983, Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_string()));
        assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
        assert_eq!(7, Solution::longest_palindrome("abcccccddd".to_string()));
        assert_eq!(1, Solution::longest_palindrome("a".to_string()));
    }
}
