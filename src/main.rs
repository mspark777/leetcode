struct Solution {}

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let &max_num = nums.iter().max().unwrap();
        let mut sieve = vec![true; (max_num + 1) as usize];
        sieve[1] = false;

        for i in 2..=Self::sqrt_usize(max_num + 1) {
            if !sieve[i] {
                continue;
            }

            for j in ((i * i)..=(max_num as usize)).step_by(i) {
                sieve[j] = false;
            }
        }

        let mut curr_value = 1;
        let mut i = 0;
        while i < nums.len() {
            let difference = nums[i] - curr_value;
            if difference < 0 {
                return false;
            }

            let diff_usize = difference as usize;
            if sieve[diff_usize] || difference == 0 {
                i += 1;
                curr_value += 1;
            } else {
                curr_value += 1;
            }
        }

        return true;
    }

    fn sqrt_usize(n: i32) -> usize {
        return (n as f64).sqrt() as usize;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![4, 9, 6, 10],
        },
        Input {
            nums: vec![6, 8, 11, 12],
        },
        Input {
            nums: vec![5, 8, 3],
        },
    ];

    for input in inputs {
        let result = Solution::prime_sub_operation(input.nums);
        println!("{result}");
    }
}
