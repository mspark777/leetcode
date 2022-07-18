mod solution;

use solution::Solution;

struct Input {
    matrix: Vec<Vec<i32>>,
    target: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            matrix: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
            target: 0,
        },
        Input {
            matrix: vec![vec![1, -1], vec![-1, 1]],
            target: 0,
        },
        Input {
            matrix: vec![vec![904]],
            target: 0,
        },
    ];

    for input in inputs {
        let result = Solution::num_submatrix_sum_target(input.matrix, input.target);
        println!("{result:?}");
    }
}
