struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut dp = HashSet::<i32>::from([0]);
        for x in stones {
            let mut dp1 = HashSet::<i32>::new();
            for s in dp.iter().copied() {
                dp1.insert(s + x);
                dp1.insert(s - x);
            }
            dp = dp1;
        }
        dp.into_iter().filter(|&x| x >= 0).min().unwrap()
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
