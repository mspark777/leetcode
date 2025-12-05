struct Solution {}

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let mut one = 0usize;
        let mut last = nums.len();

        for (i, num) in nums.iter().copied().enumerate() {
            let num = num as usize;
            if num == 1 {
                one = i;
            } else if num == nums.len() {
                last = i;
            }
        }

        if one > last {
            last += 1;
        }

        (nums.len() + one - (last + 1)) as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 1, 4, 3].to_vec(),
        },
        Input {
            nums: [2, 4, 1, 3].to_vec(),
        },
        Input {
            nums: [1, 3, 4, 2, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::semi_ordered_permutation(input.nums);
        println!("{:?}", result);
    }
}
