mod solution;

struct Input {
    target: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            target: vec![9, 3, 5],
        },
        Input {
            target: vec![1, 1, 1, 2],
        },
        Input { target: vec![8, 5] },
        Input {
            target: vec![1, 1000000000],
        },
    ];

    for input in inputs {
        let result = solution::Solution::is_possible(input.target);
        println!("{result:?}");
    }
}
