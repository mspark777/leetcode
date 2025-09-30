struct Solution {}

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.sort_unstable();
        let n = cost.len() % 3;

        cost.iter()
            .cloned()
            .enumerate()
            .fold(0, |acc, (i, c)| match (i % 3) != n {
                true => acc + c,
                _ => acc,
            })
    }
}

struct Input {
    cost: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            cost: [1, 2, 3].to_vec(),
        },
        Input {
            cost: [6, 5, 7, 9, 2, 2].to_vec(),
        },
        Input {
            cost: [5, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_cost(input.cost);
        println!("{:?}", result);
    }
}
