struct Solution {}

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut left = 0usize;
        let mut right = n - 1;
        let mut result = 0i64;

        while left < right {
            let num = Self::concat(nums[left], nums[right]);
            result += num;
            left += 1;
            right -= 1;
        }

        if (n & 1) == 1 {
            result += nums[left] as i64;
        }

        result
    }

    fn concat(left: i32, right: i32) -> i64 {
        let mut pow = 10;
        while pow <= right {
            pow *= 10
        }

        let left = (left * pow) as i64;
        left + (right as i64)
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [7, 52, 2, 4].to_vec(),
        },
        Input {
            nums: [5, 14, 13, 8, 12].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_the_array_conc_val(input.nums);
        println!("{:?}", result);
    }
}
