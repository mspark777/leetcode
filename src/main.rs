struct Solution;

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_unstable_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let half = costs.len() / 2;
        costs
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, pair)| match i < half {
                true => acc + pair[0],
                _ => acc + pair[1],
            })
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
