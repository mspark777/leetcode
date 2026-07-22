use std::collections::HashMap;

#[derive(Default)]
struct Node {
    chars: HashMap<u8, Node>,
    word: bool,
}

impl Node {
    fn insert(&mut self, s: &str) {
        let s = s.as_bytes();
        let mut n = self;

        for b in s.iter().copied() {
            n = n.chars.entry(b).or_default();
        }

        n.word = true;
    }

    fn search_with_misses(&self, word: &str, misses: usize) -> bool {
        let w = word.as_bytes();
        if w.is_empty() {
            return self.word && misses == 0;
        }

        if let Some(node) = self.chars.get(&w[0])
            && node.search_with_misses(&word[1..], misses)
        {
            return true;
        }

        if misses > 0 {
            for (&ch, node) in self.chars.iter() {
                if ch == w[0] {
                    continue;
                }

                if node.search_with_misses(&word[1..], misses - 1) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Default)]
struct MagicDictionary {
    trie: Node,
}

impl MagicDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary.iter() {
            self.trie.insert(s.as_str())
        }
    }

    fn search(&self, search_word: String) -> bool {
        self.trie.search_with_misses(&search_word, 1)
    }
}

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lengths = vec![1; n];
        let mut counts = vec![1; n];

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if (lengths[j] + 1) > lengths[i] {
                        lengths[i] = lengths[j] + 1;
                        counts[i] = 0;
                    }
                    if (lengths[j] + 1) == lengths[i] {
                        counts[i] += counts[j];
                    }
                }
            }
        }

        let max_length = lengths.iter().copied().max().unwrap();
        let mut result = 0;
        for (i, l) in lengths.into_iter().enumerate() {
            if l == max_length {
                result += counts[i];
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [1, 3, 5, 4, 7].to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_number_of_lis(input.nums);
        println!("{:?}", result);
    }
}
