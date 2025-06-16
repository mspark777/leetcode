struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .cloned()
            .fold((-1, nums[0]), |(result, premin), num| {
                if num > premin {
                    (result.max(num - premin), premin)
                } else {
                    (result, num)
                }
            })
            .0
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![7, 1, 5, 4],
        },
        Input {
            nums: vec![9, 4, 3, 2],
        },
        Input {
            nums: vec![1, 5, 2, 10],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_difference(input.nums);
        println!("{:?}", result);
    }
}
