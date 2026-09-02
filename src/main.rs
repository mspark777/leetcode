struct Solution;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        use std::collections::HashMap;
        let mut items: Vec<_> = values.into_iter().zip(labels).collect();
        items.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut count = HashMap::new();
        let mut sum = 0;
        let mut x = 0;

        for (v, l) in items {
            if x == num_wanted {
                break;
            }
            let c = count.entry(l).or_insert(0);
            if *c < use_limit {
                *c += 1;
                sum += v;
                x += 1;
            }
        }
        sum
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
