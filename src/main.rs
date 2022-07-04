mod solution;

use solution::Solution;

struct Input {
    ratings: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            ratings: vec![1, 0, 2],
        },
        Input {
            ratings: vec![1, 2, 2],
        },
        Input {
            ratings: vec![1, 2, 87, 87, 87, 2, 1],
        },
        Input {
            ratings: vec![1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::candy(input.ratings);
        println!("{result:?}");
    }
}
