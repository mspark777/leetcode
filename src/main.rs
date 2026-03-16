struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .find(|&(i, n)| Self::digit_sum(n) == i)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }

    fn digit_sum(mut n: i32) -> usize {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        sum as usize
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 3, 2].to_vec(),
        },
        Input {
            nums: [1, 10, 11].to_vec(),
        },
        Input {
            nums: [1, 2, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::smallest_index(input.nums);
        println!("{:?}", result);
    }
}
