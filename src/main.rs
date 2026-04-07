struct Solution;

impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut count = nums.len() as i32;
        let mut sum = nums.iter().copied().sum::<i32>();
        let mut result = 0;

        for num in nums {
            if count == 1 {
                break;
            }

            count -= 1;
            sum -= num;
            let average = sum / count;
            if num > average {
                result += 1;
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
            nums: [5, 4, 3].to_vec(),
        },
        Input {
            nums: [4, 1, 2].to_vec(),
        },
        Input {
            nums: [37, 96, 96].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::dominant_indices(input.nums);
        println!("{:?}", result);
    }
}
