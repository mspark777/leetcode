struct Solution {}

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut threshold = nums.iter().sum::<i32>() / 2;

        let mut nums = nums;
        nums.sort_unstable_by_key(|&n| -n);

        let mut result = Vec::<i32>::new();
        for num in nums.iter().cloned() {
            threshold -= num;
            result.push(num);
            if threshold < 0 {
                break;
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
            nums: [4, 3, 10, 9, 8].to_vec(),
        },
        Input {
            nums: [4, 4, 7, 6, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_subsequence(input.nums);
        println!("{:?}", result);
    }
}
