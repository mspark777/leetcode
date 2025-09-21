struct Solution {}

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, num) in nums.iter().cloned().enumerate() {
            let i = i as i32;
            if i % 10 == num {
                return i;
            }
        }

        return -1;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [0, 1, 2].to_vec(),
        },
        Input {
            nums: [4, 3, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::smallest_equal(input.nums);
        println!("{:?}", result);
    }
}
