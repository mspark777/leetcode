mod solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![4, 2, 3],
        },
        Input {
            nums: vec![4, 2, 1],
        },
    ];

    for input in inputs {
        let result = solution::Solution::check_possibility(input.nums);
        println!("{result:?}");
    }
}
