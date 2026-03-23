struct Solution;

impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |result, num| match num & 1 {
            0 => result | num,
            _ => result,
        })
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4, 5, 6].to_vec(),
        },
        Input {
            nums: [7, 9, 11].to_vec(),
        },
        Input {
            nums: [1, 8, 16].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::even_number_bitwise_o_rs(input.nums);
        println!("{:?}", result);
    }
}
