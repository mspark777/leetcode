struct Solution;

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut counts = [0usize; 2];
        for num in nums {
            let idx = (num & 1) as usize;
            counts[idx] += 1;
        }

        let mut result = vec![0; n];
        for n in result.iter_mut().skip(counts[0]).take(counts[1]) {
            *n = 1;
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
            nums: [4, 3, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 5, 1, 4, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::transform_array(input.nums);
        println!("{:?}", result);
    }
}
