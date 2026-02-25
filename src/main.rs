struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(Self::sum_digits).min().unwrap()
    }

    fn sum_digits(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        sum
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [10, 12, 13, 14].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [999, 19, 199].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_element(input.nums);
        println!("{:?}", result);
    }
}
