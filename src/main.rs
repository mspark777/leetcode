mod solution;

use solution::Solution;

struct Input {
    people: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            people: vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2],
            ],
        },
        Input {
            people: vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::reconstruct_queue(input.people);
        println!("{result:?}");
    }
}
