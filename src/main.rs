struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut prev = nums[0] & 1;

        for num in nums.iter().skip(1).cloned() {
            let c = num & 1;
            if prev == c {
                return false;
            } else {
                prev = c;
            }
        }

        return true;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input { nums: vec![1] },
        Input {
            nums: vec![2, 1, 4],
        },
        Input {
            nums: vec![4, 3, 1, 6],
        },
        Input {
            nums: vec![1, 6, 2],
        },
    ];

    for input in inputs {
        let result = Solution::is_array_special(input.nums);
        println!("{result:?}");
    }
}
