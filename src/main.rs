mod solution;

use solution::Solution;

struct Input {
    m: i32,
    n: i32,
    max_move: i32,
    start_row: i32,
    start_column: i32,
}

fn main() {
    let inputs = [
        Input {
            m: 2,
            n: 2,
            max_move: 2,
            start_row: 0,
            start_column: 0,
        },
        Input {
            m: 1,
            n: 3,
            max_move: 3,
            start_row: 0,
            start_column: 1,
        },
        Input {
            m: 3,
            n: 2,
            max_move: 5,
            start_row: 1,
            start_column: 1,
        },
    ];

    for input in inputs {
        let result = Solution::find_paths(
            input.m,
            input.n,
            input.max_move,
            input.start_row,
            input.start_column,
        );
        println!("{result:?}");
    }
}
