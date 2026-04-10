struct Solution;

impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 100];

        for num in nums.iter().copied() {
            if (num & 1) == 1 {
                continue;
            }

            let idx = (num as usize) - 1;
            counts[idx] += 1;
        }

        for num in nums {
            if (num & 1) == 1 {
                continue;
            }

            let idx = (num as usize) - 1;
            if counts[idx] == 1 {
                return num;
            }
        }

        -1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [3, 4, 2, 5, 4, 6].to_vec(),
    }];

    for input in inputs {
        let result = Solution::first_unique_even(input.nums);
        println!("{:?}", result);
    }
}
