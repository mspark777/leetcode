mod solution;

use solution::Solution;

struct Input {
    box_types: Vec<Vec<i32>>,
    truck_size: i32,
}

fn main() {
    let inputs = [
        Input {
            box_types: vec![vec![1, 3], vec![2, 2], vec![3, 1]],
            truck_size: 4,
        },
        Input {
            box_types: vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]],
            truck_size: 10,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_units(input.box_types, input.truck_size);
        println!("{result:?}");
    }
}
