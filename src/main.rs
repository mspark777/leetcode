struct Solution;

impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 3 {
            return vec![-1, -1];
        }

        let mut frequencies = vec![0; 100];
        for num in nums {
            let idx = (num - 1) as usize;
            frequencies[idx] += 1;
        }

        let mut x = -1;
        let mut x_frequency = -1;
        let mut y = -1;

        for (i, frequency) in frequencies.into_iter().enumerate() {
            if frequency == 0 {
                continue;
            }

            let n = (i + 1) as i32;
            if x_frequency < 0 {
                x_frequency = frequency;
                x = n;
                continue;
            }

            if frequency != x_frequency {
                y = n;
                break;
            }
        }

        match (x, y) {
            (-1, _) | (_, -1) => vec![-1, -1],
            _ => vec![x, y],
        }
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
            nums: [1, 5].to_vec(),
        },
        Input { nums: [7].to_vec() },
    ];

    for input in inputs {
        let result = Solution::min_distinct_freq_pair(input.nums);
        println!("{:?}", result);
    }
}
