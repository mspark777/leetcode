struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut changed = 0;
        let mut temp = nums.clone();

        for i in 1..nums.len() {
            if temp[i - 1] >= temp[i] {
                changed += 1;
                if i > 1 && temp[i - 2] >= temp[i] {
                    temp[i] = temp[i - 1];
                }
            }

            if changed > 1 {
                break;
            }
        }

        changed <= 1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 10, 5, 7].to_vec(),
        },
        Input {
            nums: [2, 3, 1, 2].to_vec(),
        },
        Input {
            nums: [1, 1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::can_be_increasing(input.nums);
        println!("{:?}", result);
    }
}
