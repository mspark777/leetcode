struct Solution {}

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return true;
        }

        let mut inversion_count = 0;
        for i in 0..nums.len() {
            let curr = nums[i];
            let next_idx = (i + 1) % nums.len();
            let next = nums[next_idx];
            if curr > next {
                inversion_count += 1;
            }

            if inversion_count > 1 {
                return false;
            }
        }

        return inversion_count <= 1;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 4, 5, 1, 2],
        },
        Input {
            nums: vec![2, 1, 3, 4],
        },
        Input {
            nums: vec![1, 2, 3],
        },
    ];

    for input in inputs {
        let result = Solution::check(input.nums);
        println!("{result:?}");
    }
}
