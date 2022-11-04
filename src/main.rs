use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = HashSet::<u8>::from_iter(
            [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']
                .iter()
                .cloned(),
        );

        let mut words = s.as_bytes().to_vec();
        let mut left = 0;
        let mut right = words.len() - 1;

        while left < right {
            let l = words[left];
            if !vowels.contains(&l) {
                left += 1;
                continue;
            }

            let r = words[right];
            if !vowels.contains(&r) {
                right -= 1;
                continue;
            }

            words[left] = r;
            words[right] = l;
            left += 1;
            right -= 1;
        }

        return String::from_utf8(words).unwrap();
    }
}

fn main() {
    let inputs = ["hello", "leetcode"];

    for s in inputs {
        let result = Solution::reverse_vowels(s.to_string());
        println!("{result}");
    }
}
