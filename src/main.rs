struct Solution {}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for (i, x) in nums.iter().copied().enumerate() {
            for y in nums.iter().skip(i + 1).copied() {
                if Self::beautiful(x, y) {
                    result += 1;
                }
            }
        }
        result
    }

    fn beautiful(x: i32, y: i32) -> bool {
        let first = Self::first(x);
        let last = Self::last(y);
        Self::gcd(first, last) == 1
    }

    fn gcd(x: i32, y: i32) -> i32 {
        match y > 0 {
            true => Self::gcd(y, x % y),
            _ => x,
        }
    }

    fn first(i: i32) -> i32 {
        match i > 9 {
            true => Self::first(i / 10),
            _ => i,
        }
    }

    fn last(i: i32) -> i32 {
        i % 10
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 5, 1, 4].to_vec(),
        },
        Input {
            nums: [11, 21, 12].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_beautiful_pairs(input.nums);
        println!("{:?}", result);
    }
}
