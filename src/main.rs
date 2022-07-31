mod solution;

use solution::NumArray;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![Input {
        nums: vec![1, 3, 5],
    }];

    for input in inputs {
        let mut narr = NumArray::new(input.nums);
        println!("{:?}", narr.sum_range(0, 2));
        narr.update(1, 2);
        println!("{:?}", narr.sum_range(0, 2));
    }
}
