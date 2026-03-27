struct Solution;

impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = [0; 101];
        for num in nums {
            let idx = num as usize;
            counts[idx] += 1;
        }

        let mut result = 0;
        for (i, count) in counts.into_iter().enumerate().skip(1) {
            if (count > 0) && ((count % k) == 0) {
                result += (i as i32) * count;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 2, 3, 3, 3, 3, 4].to_vec(),
            k: 2,
        },
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
            k: 2,
        },
        Input {
            nums: [4, 4, 4, 1, 2, 3].to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::sum_divisible_by_k(input.nums, input.k);
        println!("{:?}", result);
    }
}
