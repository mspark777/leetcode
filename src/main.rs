struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        nums.chunks(3)
            .map(|chunk| {
                if (chunk[2] - chunk[0]) <= k {
                    Some(chunk.to_vec())
                } else {
                    None
                }
            })
            .collect::<Option<Vec<Vec<i32>>>>()
            .unwrap_or_default()
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 3, 4, 8, 7, 9, 3, 5, 1],
            k: 2,
        },
        Input {
            nums: vec![2, 4, 2, 2, 5, 2],
            k: 2,
        },
        Input {
            nums: vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11],
            k: 14,
        },
    ];

    for input in inputs {
        let result = Solution::divide_array(input.nums, input.k);
        println!("{:?}", result);
    }
}
