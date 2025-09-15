struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
        for num in nums.iter().cloned() {
            counts
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut result = 0;
        for num in counts.keys().cloned() {
            if let Some(target) = counts.get(&(num + k)) {
                result += counts.get(&num).unwrap() * target;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 2, 1].to_vec(),
            k: 1,
        },
        Input {
            nums: [1, 3].to_vec(),
            k: 3,
        },
        Input {
            nums: [3, 2, 1, 5, 4].to_vec(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::count_k_difference(input.nums, input.k);
        println!("{:?}", result);
    }
}
