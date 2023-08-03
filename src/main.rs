mod utils;

use std::{char, collections::HashMap};

use utils::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.as_bytes();
        if digits.is_empty() {
            return vec![];
        }

        let mut letters_map = HashMap::<u8, &'static str>::with_capacity(8);
        letters_map.insert(b'2', "abc");
        letters_map.insert(b'3', "def");
        letters_map.insert(b'4', "ghi");
        letters_map.insert(b'5', "jkl");
        letters_map.insert(b'6', "mno");
        letters_map.insert(b'7', "pqrs");
        letters_map.insert(b'8', "tuv");
        letters_map.insert(b'9', "wxyz");

        let mut stack = vec!["".to_string()];
        let mut result = Vec::<String>::new();

        while let Some(top) = stack.pop() {
            let code = digits[top.len()];
            let letters = *(letters_map.get(&code).unwrap());
            for &letter in letters.as_bytes() {
                let ch = char::from_u32(letter as u32).unwrap();
                let combination = format!("{top}{ch}");
                if combination.len() == digits.len() {
                    result.push(combination);
                } else {
                    stack.push(combination);
                }
            }
        }

        return result;
    }
}

fn main() {
    let inputs = ["23", "", "2"];

    for input in inputs {
        let result = Solution::letter_combinations(input.to_string());
        println!("{result:?}");
    }
}
