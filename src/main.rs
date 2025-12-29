struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = nums.len();

        for i in 0..n {
            let mut set = 0u128;
            for num in nums.iter().skip(i) {
                set |= 1 << num;
                let distinct = set.count_ones() as i32;
                result += distinct * distinct;
            }
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
            nums: [1, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sum_counts(input.nums);
        println!("{:?}", result);
    }
}
