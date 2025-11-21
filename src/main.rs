struct Solution {}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        for (i, row) in nums.iter().enumerate() {
            let left = row[i];
            if (left > result) && Self::is_prime(left) {
                result = left;
            }

            let right = row[n - i - 1];
            if (right > result) && Self::is_prime(right) {
                result = right;
            }
        }

        result
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        } else if n == 2 {
            return true;
        } else if (n & 1) == 0 {
            return false;
        }

        let n = n as i64;
        for i in (3..n).step_by(2) {
            let d = i * i;
            if d > n {
                break;
            } else if (n % i) == 0 {
                return false;
            }
        }

        true
    }
}

struct Input {
    nums: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            nums: [[1, 2, 3], [5, 6, 7], [9, 10, 11]]
                .map(|a| a.to_vec())
                .to_vec(),
        },
        Input {
            nums: [[1, 2, 3], [5, 17, 7], [9, 11, 10]]
                .map(|a| a.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::diagonal_prime(input.nums);
        println!("{:?}", result);
    }
}
