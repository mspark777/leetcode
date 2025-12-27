struct Solution;

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen_idx = -1i32;
        let mut seen = Vec::<i32>::new();
        let mut result = Vec::<i32>::new();

        for num in nums {
            if num == -1 {
                if seen_idx >= 0 {
                    result.push(seen.get(seen_idx as usize).copied().unwrap_or(-1));
                } else {
                    result.push(-1);
                }
                seen_idx -= 1;
            } else {
                seen.push(num);
                seen_idx = (seen.len() as i32) - 1;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, -1, -1, -1].to_vec(),
        },
        Input {
            nums: [1, -1, 2, -1, -1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::last_visited_integers(input.nums);
        println!("{:?}", result);
    }
}
