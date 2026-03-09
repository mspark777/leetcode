struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let mut frequencies = vec![0; 51];
        for (i, num) in nums.iter().copied().enumerate() {
            let i = i as i32;
            frequencies[num as usize] += k.min(i + 1).min(n - i);
        }

        for (i, frequency) in frequencies.into_iter().enumerate().rev() {
            if (frequency == 1) || ((k == n) && (frequency > 0)) {
                return i as i32;
            }
        }

        -1
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 9, 2, 1, 7].to_vec(),
            k: 3,
        },
        Input {
            nums: [3, 9, 7, 2, 1, 7].to_vec(),
            k: 4,
        },
        Input {
            nums: [0, 0].to_vec(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::largest_integer(input.nums, input.k);
        println!("{:?}", result);
    }
}
