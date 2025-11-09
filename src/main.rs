struct Solution {}

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut element_sum = 0;
        let mut digit_sum = 0;
        for num in nums.iter().cloned() {
            element_sum += num;
            digit_sum += Self::digit_sum(num);
        }

        (element_sum - digit_sum).abs()
    }

    fn digit_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
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
            nums: [1, 15, 6, 3].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::difference_of_sum(input.nums);
        println!("{:?}", result);
    }
}
