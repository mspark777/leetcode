struct Solution {}

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let mut pair_weights = vec![0; n - 1];
        for i in 0..(n - 1) {
            pair_weights[i] = weights[i] + weights[i + 1];
        }

        pair_weights.sort_unstable();
        let mut result = 0i64;

        let k = k as usize;
        for i in 0..(k - 1) {
            let diff = pair_weights[n - 2 - i] - pair_weights[i];
            result += diff as i64;
        }
        return result;
    }
}

struct Input {
    weights: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            weights: vec![1, 3, 5, 1],
            k: 2,
        },
        Input {
            weights: vec![1, 3],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::put_marbles(input.weights, input.k);
        println!("{result:?}");
    }
}
