struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut counts = vec![0; 100];
        for num in nums {
            let idx = (num - 1) as usize;
            counts[idx] += 1;
            if counts[idx] > 2 {
                return false;
            }
        }

        true
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 1, 2, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [1, 1, 1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::is_possible_to_split(input.nums);
        println!("{:?}", result);
    }
}
