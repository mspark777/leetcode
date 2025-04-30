struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums.iter().cloned() {
            if (num >= 10) && (num <= 99) {
                result += 1;
            } else if (num >= 1000) && (num <= 9999) {
                result += 1;
            } else if num == 100000 {
                result += 1;
            }
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![12, 345, 2, 6, 7896],
        },
        Input {
            nums: vec![555, 901, 482, 1771],
        },
    ];

    for input in inputs {
        let result = Solution::find_numbers(input.nums);
        println!("{result:?}");
    }
}
