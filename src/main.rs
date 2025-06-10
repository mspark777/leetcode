struct Solution {}

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut counts = std::collections::HashMap::<char, i32>::with_capacity(26);
        let mut min_even = 0;
        for ch in s.chars() {
            counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
            min_even += 1;
        }

        let mut max_odd = 1;
        for count in counts.values().cloned() {
            if count & 1 == 1 {
                max_odd = max_odd.max(count);
            } else {
                min_even = min_even.min(count);
            }
        }

        return max_odd - min_even;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![Input { s: "aaaaabbc" }, Input { s: "abcabcab" }];

    for input in inputs {
        let result = Solution::max_difference(input.s.to_string());
        println!("{:?}", result);
    }
}
