struct Solution;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;

        let mut single = HashMap::<i32, i32>::new();
        let mut pairs = HashMap::<i32, i32>::new();
        const MOD: i32 = 1_000_000_007;
        let mut result = 0;

        for x in arr {
            result =
                (result + pairs.get(&(target - x)).copied().unwrap_or_default()).rem_euclid(MOD);

            for (&y, &k) in single.iter() {
                pairs
                    .entry(x + y)
                    .and_modify(|n| *n = (*n + k).rem_euclid(MOD))
                    .or_insert(k.rem_euclid(MOD));
            }

            single.entry(x).and_modify(|n| *n += 1).or_insert(1);
        }
        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "())".to_string(),
    }];

    for input in inputs {
        let result = Solution::min_add_to_make_valid(input.s);
        println!("{:?}", result);
    }
}
