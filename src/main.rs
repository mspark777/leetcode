mod solution;

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![3, 2, 1, 5, 6, 4],
            k: 2,
        },
        Input {
            nums: vec![3, 2, 3, 1, 2, 4, 5, 5, 6],
            k: 4,
        },
    ];

    for input in inputs {
        let result = solution::Solution::find_kth_largest(input.nums, input.k);
        println!("{result:?}");
    }
}
