struct Solution {}

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pos = vec![-1; 31];
        let mut result = vec![0; n];

        for (i, num) in nums.iter().cloned().enumerate().rev() {
            let mut j = i;
            for bit in 0..31usize {
                if (num & (1 << bit)) == 0 {
                    if pos[bit] != -1 {
                        j = j.max(pos[bit] as usize);
                    }
                } else {
                    pos[bit] = i as i32;
                }
            }

            result[i] = (j + 1 - i) as i32;
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
            nums: [1, 0, 2, 1, 3].to_vec(),
        },
        Input {
            nums: [1, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::smallest_subarrays(input.nums);
        println!("{:?}", result);
    }
}
