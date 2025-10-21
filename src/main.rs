struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0; 101];

        for num in nums.iter().cloned() {
            let idx = num as usize;
            counts[idx] += 1;
        }

        let mut pairs = 0;
        let mut left = nums.len() as i32;

        for count in counts.iter().cloned() {
            let pair = count / 2;
            pairs += pair;
            left -= pair * 2;
        }

        vec![pairs, left]
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 3, 2, 1, 3, 2, 2].to_vec(),
        },
        Input {
            nums: [1, 1].to_vec(),
        },
        Input { nums: [0].to_vec() },
    ];

    for input in inputs {
        let result = Solution::number_of_pairs(input.nums);
        println!("{:?}", result);
    }
}
