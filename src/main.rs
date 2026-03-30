struct Solution;

impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_by(|&a, &b| Self::compare(a as u32, b as u32));
        nums
    }

    fn compare(left: u32, right: u32) -> std::cmp::Ordering {
        let l = left.reverse_bits() >> left.leading_zeros();
        let r = right.reverse_bits() >> right.leading_zeros();

        match l.cmp(&r) {
            std::cmp::Ordering::Equal => left.cmp(&right),
            ord => ord,
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [4, 5, 4].to_vec(),
        },
        Input {
            nums: [3, 6, 5, 8].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sort_by_reflection(input.nums);
        println!("{:?}", result);
    }
}
