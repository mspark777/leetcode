struct Solution;

impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let mut a = i32::MIN;
        let mut b = i32::MIN;
        let mut c = i32::MAX;

        for num in nums {
            if a < num {
                b = a;
                a = num;
            } else if b < num {
                b = num;
            }

            if c > num {
                c = num;
            }
        }

        a + b - c
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 4, 2, 5].to_vec(),
        },
        Input {
            nums: [-2, 0, 5, -2, 4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::maximize_expression_of_three(input.nums);
        println!("{:?}", result);
    }
}
