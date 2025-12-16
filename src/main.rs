struct Solution {}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let max = nums.iter().copied().max().unwrap() as usize;
        if nums.len() != (max + 1) {
            return false;
        }

        let mut counts = vec![0; nums.len()];
        for num in nums.iter().copied() {
            let num = num as usize;
            counts[num] += 1;
            if (num != max) && (counts[num] > 1) {
                return false;
            }
        }

        counts[max] == 2
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 1, 3].to_vec(),
        },
        Input {
            nums: [1, 3, 3, 2].to_vec(),
        },
        Input {
            nums: [1, 1].to_vec(),
        },
        Input {
            nums: [3, 4, 4, 1, 2, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::is_good(input.nums);
        println!("{:?}", result);
    }
}
