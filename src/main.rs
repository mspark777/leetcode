struct Solution {}

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let &max_value = nums.iter().max().unwrap();
        let mut counts = vec![0; (max_value + 1) as usize];

        for &num in nums.iter() {
            counts[0.max(num - k) as usize] += 1;

            if (num + k + 1) <= max_value {
                counts[(num + k + 1) as usize] -= 1;
            }
        }

        let mut result = 0;
        let mut current_sum = 0;
        for count in counts.iter() {
            current_sum += count;
            result = result.max(current_sum);
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![4, 6, 1, 2],
            k: 2,
        },
        Input {
            nums: vec![1, 1, 1, 1],
            k: 10,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_beauty(input.nums, input.k);
        println!("{result}");
    }
}
