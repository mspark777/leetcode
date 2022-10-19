use std::{cmp::Ordering, collections::HashMap};

struct Node<'a> {
    word: &'a String,
    count: i32,
}

struct Solution {}
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counts = HashMap::<&String, i32>::new();

        for word in words.iter() {
            if let Some(count) = counts.get_mut(word) {
                *count += 1;
            } else {
                counts.insert(word, 1);
            }
        }

        let mut heap = Vec::<Node>::with_capacity(counts.len());
        for (word, count) in counts.iter() {
            heap.push(Node {
                word: *word,
                count: *count,
            });
        }

        heap.sort_unstable_by(|a, b| {
            let c = b.count.cmp(&a.count);
            if c != Ordering::Equal {
                return c;
            }

            return a.word.cmp(b.word);
        });

        return heap
            .iter()
            .take(k as usize)
            .map(|n| n.word.clone())
            .collect();
    }
}

struct Input {
    words: Vec<&'static str>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            words: vec!["i", "love", "leetcode", "i", "love", "coding"],
            k: 2,
        },
        Input {
            words: vec![
                "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
            ],
            k: 4,
        },
    ];

    for Input { words, k } in inputs {
        let words = words.iter().map(|s| s.to_string()).collect();
        let result = Solution::top_k_frequent(words, k);
        println!("{result:?}");
    }
}
