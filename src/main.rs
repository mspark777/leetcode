/*
leetcode
 */

use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut group_map = HashMap::<u8, HashSet<&str>>::new();

        for idea in ideas.iter() {
            let s = idea.as_str();
            let first = s.bytes().nth(0).unwrap();
            let remains = &s[1..];

            group_map
                .entry(first)
                .or_insert(HashSet::new())
                .insert(remains);
        }

        let mut result = 0i64;
        let groups = Vec::from_iter(group_map.values());

        for i in 0..(groups.len() - 1) {
            let cur = groups[i];
            for j in (i + 1)..groups.len() {
                let group = groups[j];
                let mut num = 0i64;

                for &idea in cur.iter() {
                    if group.contains(idea) {
                        num += 1;
                    }
                }

                let clen = cur.len() as i64;
                let glen = group.len() as i64;
                result += 2 * (clen - num) * (glen - num)
            }
        }

        return result;
    }
}

fn main() {
    let inputs: Vec<Vec<&'static str>> = vec![
        vec!["coffee", "donuts", "time", "toffee"],
        vec!["lack", "back"],
    ];

    for input in inputs {
        let result = Solution::distinct_names(input.iter().map(|s| s.to_string()).collect());
        println!("{result}");
    }
}
