struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut sub_domain_visit_counter = HashMap::<&str, usize>::new();

        for cpdomain in cpdomains.iter() {
            let (rep, sub_domain) = cpdomain.split_once(' ').unwrap();
            let rep = rep.parse::<usize>().unwrap();

            for (i, ch) in sub_domain.char_indices() {
                if ch != '.' {
                    continue;
                }

                let domain = &sub_domain[(i + 1)..];
                sub_domain_visit_counter
                    .entry(domain)
                    .and_modify(|e| *e += rep)
                    .or_insert(rep);
            }

            sub_domain_visit_counter
                .entry(sub_domain)
                .and_modify(|e| *e += rep)
                .or_insert(rep);
        }

        sub_domain_visit_counter
            .into_iter()
            .map(|(key, value)| format!("{} {}", value, key))
            .collect()
    }
}

struct Input {
    cpdomains: Vec<String>,
}

fn main() {
    let inputs = [Input {
        cpdomains: ["9001 discuss.leetcode.com"]
            .map(|v| v.to_string())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::subdomain_visits(input.cpdomains);
        println!("{:?}", result);
    }
}
