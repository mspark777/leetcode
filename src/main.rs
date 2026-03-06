struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let result = match nums.iter().sum::<i32>() & 1 {
            0 => nums.len() - 1,
            _ => 0,
        };

        result as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [10, 10, 3, 7, 6].to_vec(),
        },
        Input {
            nums: [1, 2, 2].to_vec(),
        },
        Input {
            nums: [2, 4, 6, 8].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_partitions(input.nums);
        println!("{:?}", result);
    }
}
