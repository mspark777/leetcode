struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = vec![0; 32];
        for mut num in nums.iter().copied() {
            let mut i = 0usize;
            while num > 0 {
                bits[i] += num & 1;
                num >>= 1;
                i += 1;
            }
        }

        let mut result = 0;
        for (i, b) in bits.into_iter().enumerate() {
            if b >= k {
                result |= 1 << i;
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
            nums: [7, 12, 9, 8, 9, 15].to_vec(),
            k: 4,
        },
        Input {
            nums: [2, 12, 1, 11, 4, 5].to_vec(),
            k: 6,
        },
        Input {
            nums: [10, 8, 5, 9, 11, 6, 8].to_vec(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::find_k_or(input.nums, input.k);
        println!("{:?}", result);
    }
}
