mod solution;

use solution::Solution;

struct Input {
    houses: Vec<i32>,
    cost: Vec<Vec<i32>>,
    m: i32,
    n: i32,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            houses: vec![0, 0, 0, 0, 0],
            cost: vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1],
            ],
            m: 5,
            n: 2,
            target: 3,
        },
        Input {
            houses: vec![0, 2, 1, 2, 0],
            cost: vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1],
            ],
            m: 5,
            n: 2,
            target: 3,
        },
        Input {
            houses: vec![3, 1, 2, 3],
            cost: vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            m: 4,
            n: 3,
            target: 3,
        },
    ];

    for input in inputs {
        let result = Solution::min_cost(input.houses, input.cost, input.m, input.n, input.target);
        println!("{result:?}");
    }
}
