mod solution;

use solution::Solution;

struct Input {
    matrix: Vec<Vec<i32>>,
    target: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            matrix: vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            target: 5,
        },
        Input {
            matrix: vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            target: 20,
        },
    ];

    for input in inputs {
        let result = Solution::search_matrix(input.matrix, input.target);
        println!("{result:?}");
    }
}
