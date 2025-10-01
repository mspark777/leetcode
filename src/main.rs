struct Solution {}

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 1 {
            return 0;
        }

        let mut min = nums[0];
        let mut max = nums[0];
        let mut min_appear = 1usize;
        let mut max_appear = 1usize;

        for num in nums.iter().cloned().skip(1) {
            if num < min {
                min = num;
                min_appear = 1;
            } else if num == min {
                min_appear += 1;
            }

            if max < num {
                max = num;
                max_appear = 1;
            } else if max == num {
                max_appear += 1;
            }
        }

        let result = match min == max {
            true => 0,
            _ => n - min_appear - max_appear,
        };

        result as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [11, 7, 2, 15].to_vec(),
        },
        Input {
            nums: [-3, 3, 3, 90].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_elements(input.nums);
        println!("{:?}", result);
    }
}
