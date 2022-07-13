mod solution;

use solution::Solution;

struct Input {
    row_index: i32,
}

fn main() {
    let inputs = [
        Input { row_index: 3 },
        Input { row_index: 0 },
        Input { row_index: 1 },
    ];

    for input in inputs {
        let result = Solution::get_row(input.row_index);
        println!("{result:?}");
    }
}
