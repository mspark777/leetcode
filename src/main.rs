struct Solution;

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut counts = [0; 101];
        for num in nums {
            let idx = num as usize;
            counts[idx] += 1;
        }

        counts
            .into_iter()
            .find(|&count| Self::check(count))
            .is_some()
    }

    fn check(count: i32) -> bool {
        let d = Self::divable;
        match count {
            2 | 3 | 5 | 7 => true,
            _ if (count == 1) || d(count, 2) || d(count, 3) || d(count, 5) || d(count, 7) => false,
            _ => true,
        }
    }

    fn divable(left: i32, right: i32) -> bool {
        (left % right) == 0
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4, 5, 4].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            nums: [2, 2, 2, 4, 4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::check_prime_frequency(input.nums);
        println!("{:?}", result);
    }
}
