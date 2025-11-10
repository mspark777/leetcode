struct Solution {}

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut digits = Vec::<i32>::new();
        for num in nums.iter().rev().cloned() {
            let mut n = num;
            while n > 0 {
                let digit = n % 10;
                n /= 10;
                digits.push(digit);
            }
        }

        digits.iter().rev().cloned().collect()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [13, 25, 83, 77].to_vec(),
        },
        Input {
            nums: [7, 1, 3, 9].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::separate_digits(input.nums);
        println!("{:?}", result);
    }
}
