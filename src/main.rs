struct Solution;

impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let n = cost.len();
        let mut result = vec![cost[0]; n];
        for i in 1..n {
            result[i] = result[i - 1].min(cost[i]);
        }

        result
    }
}

struct Input {
    cost: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            cost: [5, 3, 4, 1, 3, 2].to_vec(),
        },
        Input {
            cost: [1, 2, 4, 6, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_costs(input.cost);
        println!("{:?}", result);
    }
}
