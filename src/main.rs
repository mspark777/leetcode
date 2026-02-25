struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        for (i, n) in nums.into_iter().enumerate() {
            let mut ans = -1;
            let mut d = 1;
            while (n & d) != 0 {
                ans = n - d;
                d <<= 1;
            }
            result[i] = ans;
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
            nums: [2, 3, 5, 7].to_vec(),
        },
        Input {
            nums: [11, 13, 31].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_bitwise_array(input.nums);
        println!("{:?}", result);
    }
}
