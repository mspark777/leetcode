struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut result = 0;
        for i in 0..32 {
            let mut count = 0;
            for num in nums.iter().copied() {
                count += (num >> i) & 1;
            }
            result += count * (n - count);
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
            nums: [4, 14, 2].to_vec(),
        },
        Input {
            nums: [4, 14, 4].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::total_hamming_distance(input.nums);
        println!("{:?}", result);
    }
}
