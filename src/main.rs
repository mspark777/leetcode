struct Solution {}

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        nums.iter()
            .skip(1)
            .fold((1, nums[0]), |(result, prev), &n| {
                if (n - prev) > k {
                    (result + 1, n)
                } else {
                    (result, prev)
                }
            })
            .0
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 6, 1, 2, 5],
            k: 2,
        },
        Input {
            nums: vec![1, 2, 3],
            k: 1,
        },
        Input {
            nums: vec![2, 2, 4, 5],
            k: 0,
        },
    ];

    for input in inputs {
        let result = Solution::partition_array(input.nums, input.k);
        println!("{:?}", result);
    }
}
