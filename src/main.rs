struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut result = 0;
        for (i, num) in nums.into_iter().enumerate() {
            if i.count_ones() == k {
                result += num;
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
            nums: [5, 10, 1, 5, 2].to_vec(),
            k: 1,
        },
        Input {
            nums: [4, 3, 2, 1].to_vec(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::sum_indices_with_k_set_bits(input.nums, input.k);
        println!("{:?}", result);
    }
}
