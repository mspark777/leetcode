struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut flags = 0u64;
        let mut result = 0;
        for num in nums {
            let flag = 1u64 << num;
            match flags & flag {
                0 => flags |= flag,
                _ => result ^= num,
            };
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 1, 3].to_vec(),
        },
        Input {
            nums: [1, 2, 3].to_vec(),
        },
        Input {
            nums: [1, 2, 2, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::duplicate_numbers_xor(input.nums);
        println!("{:?}", result);
    }
}
