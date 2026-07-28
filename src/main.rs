struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Node {
    children: HashMap<char, Node>,
    flag: bool,
}

impl Node {
    fn insert(&mut self, word: String, best: &mut String) {
        let mut node = self;
        let mut valid = true;

        for (i, c) in word.chars().enumerate() {
            node = node.children.entry(c).or_default();
            if i == word.len() - 1 {
                node.flag = true;
            }

            if !node.flag {
                valid = false;
            }
        }

        if !valid {
            return;
        }

        if best.len() < word.len() || (best.len() == word.len() && word.as_str() < best.as_str()) {
            best.clear();
            best.extend(word.chars());
        }
    }
}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut words = words;
        let mut trie = Node::default();
        let mut best = String::new();

        words.sort_by(|a, b| a.len().cmp(&b.len()));
        for word in words {
            trie.insert(word, &mut best);
        }

        best
    }
}

struct Input {
    prices: Vec<i32>,
    fee: i32,
}

fn main() {
    let inputs = [Input {
        prices: [1, 3, 2, 8, 4, 9].to_vec(),
        fee: 2,
    }];

    for input in inputs.into_iter() {
        let result = Solution::max_profit(input.prices, input.fee);
        println!("{:?}", result);
    }
}
