struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut nums = s
            .chars()
            .map(|ch| (ch as i32) - ('0' as i32))
            .collect::<Vec<i32>>();

        while nums.len() > 2 {
            nums = nums
                .iter()
                .copied()
                .zip(nums.iter().copied().skip(1))
                .map(|(left, right)| (left + right) % 10)
                .collect();
        }

        nums[0] == nums[1]
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "3902".to_string(),
        },
        Input {
            s: "34789".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::has_same_digits(input.s);
        println!("{:?}", result);
    }
}
