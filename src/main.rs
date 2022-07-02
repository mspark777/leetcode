mod solution;

use solution::Solution;

struct Input {
    h: i32,
    w: i32,
    horizontal_cuts: Vec<i32>,
    vertical_cuts: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            h: 5,
            w: 4,
            horizontal_cuts: vec![1, 2, 4],
            vertical_cuts: vec![1, 3],
        },
        Input {
            h: 5,
            w: 4,
            horizontal_cuts: vec![3, 1],
            vertical_cuts: vec![1],
        },
        Input {
            h: 5,
            w: 4,
            horizontal_cuts: vec![3],
            vertical_cuts: vec![3],
        },
        Input {
            h: 1000000000,
            w: 1000000000,
            horizontal_cuts: vec![2],
            vertical_cuts: vec![2],
        },
    ];

    for input in inputs {
        let result =
            Solution::max_area(input.h, input.w, input.horizontal_cuts, input.vertical_cuts);
        println!("{result:?}");
    }
}
