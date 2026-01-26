struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut len = usize::MAX;

        for i in 0..nums.len() {
            let mut current = 0;
            for (j, num) in nums.iter().copied().enumerate().skip(i) {
                current |= num;
                if current >= k {
                    len = len.min(j + 1 - i);
                    break;
                }
            }
        }

        match len {
            usize::MAX => -1,
            _ => len as i32,
        }
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3].to_vec(),
            k: 2,
        },
        Input {
            nums: [2, 1, 8].to_vec(),
            k: 10,
        },
        Input {
            nums: [1, 2].to_vec(),
            k: 0,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_subarray_length(input.nums, input.k);
        println!("{:?}", result);
    }
}
