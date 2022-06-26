mod solution;

struct Input {
    card_points: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            card_points: vec![1, 2, 3, 4, 5, 6, 1],
            k: 3,
        },
        Input {
            card_points: vec![2, 2, 2],
            k: 2,
        },
        Input {
            card_points: vec![9, 7, 7, 9, 7, 7, 9],
            k: 7,
        },
    ];

    for input in inputs {
        let result = solution::Solution::max_score(input.card_points, input.k);
        println!("{result:?}");
    }
}
