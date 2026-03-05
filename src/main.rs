struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for (i, num) in nums.iter().copied().enumerate() {
            let j = i as i32;
            let start = 0.max(j - num);
            result += nums
                .iter()
                .skip(start as usize)
                .take((j - start + 1).max(0) as usize)
                .sum::<i32>();
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
            nums: [2, 3, 1].to_vec(),
        },
        Input {
            nums: [3, 1, 1, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::subarray_sum(input.nums);
        println!("{:?}", result);
    }
}
