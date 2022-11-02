use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

struct Solution {}
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let bank_set = HashSet::<String>::from_iter(bank.iter().cloned());
        let mut seens = HashSet::<String>::with_capacity(bank.len());
        let mut queue = VecDeque::<String>::with_capacity(bank.len());
        let mut result = 0;

        seens.insert(start.clone());
        queue.push_back(start.clone());

        while !queue.is_empty() {
            let queue_len = queue.len();
            for _ in 0..queue_len {
                let gene = queue.pop_front().unwrap();
                if gene == end {
                    return result;
                }

                let temp: Vec<char> = gene.chars().collect();
                for g in "ACGT".chars() {
                    for j in 0..temp.len() {
                        let mut genes = temp.clone();
                        genes[j] = g;

                        let neighbor: String = genes.iter().collect();

                        if !seens.contains(&neighbor) && bank_set.contains(&neighbor) {
                            queue.push_back(neighbor.clone());
                            seens.insert(neighbor.clone());
                        }
                    }
                }
            }

            result += 1;
        }

        return -1;
    }
}

struct Input {
    start: &'static str,
    end: &'static str,
    bank: Vec<&'static str>,
}

fn main() {
    let inputs = [
        Input {
            start: "AACCGGTT",
            end: "AACCGGTA",
            bank: vec!["AACCGGTA"],
        },
        Input {
            start: "AACCGGTT",
            end: "AAACGGTA",
            bank: vec!["AACCGGTA", "AACCGCTA", "AAACGGTA"],
        },
        Input {
            start: "AAAAACCC",
            end: "AACCCCCC",
            bank: vec!["AAAACCCC", "AAACCCCC", "AACCCCCC"],
        },
        Input {
            start: "AACCGGTT",
            end: "AACCGCTA",
            bank: vec!["AACCGGTA", "AACCGCTA", "AAACGGTA"],
        },
    ];

    for Input { start, end, bank } in inputs {
        let start = start.to_string();
        let end = end.to_string();
        let bank = bank.iter().map(|s| s.to_string()).collect();
        let result = Solution::min_mutation(start, end, bank);
        println!("{result}");
    }
}
