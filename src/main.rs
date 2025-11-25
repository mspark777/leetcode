struct Solution {}

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut max_score = i32::MIN;

        for divisor in divisors.iter().copied() {
            let score = nums.iter().fold(0, |acc, &n| match n % divisor {
                0 => acc + 1,
                _ => acc,
            });

            if (score > max_score) || ((score == max_score) && (divisor < result)) {
                result = divisor;
                max_score = score;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    divisors: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 9, 15, 50].to_vec(),
            divisors: [5, 3, 7, 2].to_vec(),
        },
        Input {
            nums: [4, 7, 9, 3, 9].to_vec(),
            divisors: [5, 2, 3].to_vec(),
        },
        Input {
            nums: [20, 14, 21, 10].to_vec(),
            divisors: [10, 16, 20].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_div_score(input.nums, input.divisors);
        println!("{:?}", result);
    }
}
