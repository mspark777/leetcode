struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut queue = std::collections::BinaryHeap::from_iter(nums.iter().map(|i| {
            let i = (*i) as i64;
            return std::cmp::Reverse(i);
        }));

        let mut result = 0;

        let k = k as i64;
        while let Some(&std::cmp::Reverse(top)) = queue.peek() {
            if top >= k {
                break;
            }

            let left = queue.pop();
            if left.is_none() {
                break;
            }

            let right = queue.pop();
            if right.is_none() {
                break;
            }

            let std::cmp::Reverse(left) = left.unwrap();
            let std::cmp::Reverse(right) = right.unwrap();

            queue.push(std::cmp::Reverse(left * 2 + right));
            result += 1;
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
            nums: vec![2, 11, 10, 1, 3],
            k: 10,
        },
        Input {
            nums: vec![1, 1, 2, 4, 9],
            k: 20,
        },
        Input {
            nums: vec![
                1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999,
            ],
            k: 1000000000,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{result:?}");
    }
}
