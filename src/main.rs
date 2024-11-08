struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor_product = nums.iter().fold(0, |acc, n| acc ^ n);
        let mut result = Vec::<i32>::with_capacity(nums.len());

        let mask = (1 << maximum_bit) - 1;
        for num in nums.iter().rev() {
            result.push(mask ^ xor_product);
            xor_product ^= num;
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    maximum_bit: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![0, 1, 1, 3],
            maximum_bit: 2,
        },
        Input {
            nums: vec![2, 3, 4, 7],
            maximum_bit: 2,
        },
        Input {
            nums: vec![0, 1, 2, 2, 5, 7],
            maximum_bit: 3,
        },
    ];

    for input in inputs {
        let result = Solution::get_maximum_xor(input.nums, input.maximum_bit);
        println!("{result:?}");
    }
}
