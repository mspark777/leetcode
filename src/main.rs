struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut result = 0;
        let mut sum = 0;
        let mut sums = HashMap::<i32, i32>::new();
        sums.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(cnt) = sums.get(&(sum - k)) {
                result += cnt;
            }

            sums.entry(sum).and_modify(|cnt| *cnt += 1).or_insert(1);
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
            nums: [1, 1, 1].to_vec(),
            k: 2,
        },
        Input {
            nums: [1, 2, 3].to_vec(),
            k: 3,
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::subarray_sum(input.nums, input.k);
        println!("{:?}", result);
    }
}
