struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let n = nums.len();
        let k = k as usize;
        let x = x as usize;
        let mut result = Vec::<i32>::with_capacity(n + 1 - k);

        for i in 0..=(n - k) {
            let mut counts = HashMap::<i32, i32>::new();
            for n in nums.iter().copied().skip(i).take(k) {
                counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
            }

            let mut frequencies = counts
                .into_iter()
                .map(|(num, count)| (count, num))
                .collect::<Vec<_>>();
            frequencies.sort_unstable_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
            let xsum: i32 = frequencies
                .into_iter()
                .take(x)
                .map(|(count, num)| count * num)
                .sum();

            result.push(xsum);
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
    x: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 1, 2, 2, 3, 4, 2, 3].to_vec(),
            k: 6,
            x: 2,
        },
        Input {
            nums: [3, 8, 7, 8, 7, 5].to_vec(),
            k: 2,
            x: 2,
        },
    ];

    for input in inputs {
        let result = Solution::find_x_sum(input.nums, input.k, input.x);
        println!("{:?}", result);
    }
}
