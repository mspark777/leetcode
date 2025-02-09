struct Solution {}

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;
        let mut diff_counts = std::collections::HashMap::<i64, i64>::new();

        for (i, num) in nums.iter().cloned().enumerate() {
            let i = i as i64;
            let diff = i - (num as i64);
            let &good_paris_count = diff_counts.get(&diff).unwrap_or(&0);
            result += (i - good_paris_count) as i64;

            diff_counts.entry(diff).and_modify(|e| *e += 1).or_insert(1);
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![4, 1, 3, 3],
        },
        Input {
            nums: vec![1, 2, 3, 4, 5],
        },
    ];

    for input in inputs {
        let result = Solution::count_bad_pairs(input.nums);
        println!("{result:?}");
    }
}
