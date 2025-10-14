struct Solution {}

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = std::collections::HashMap::<i32, i32>::new();
        for ns in nums.iter() {
            for n in ns.iter().cloned() {
                counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let n = nums.len() as i32;
        let mut result = Vec::<i32>::with_capacity(nums.len());
        for (&num, &cnt) in counts.iter() {
            if cnt == n {
                result.push(num);
            }
        }

        result.sort_unstable();
        result.shrink_to_fit();
        result
    }
}

struct Input {
    nums: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]],
        },
        Input {
            nums: vec![vec![1, 2, 3], vec![4, 5, 6]],
        },
    ];

    for input in inputs {
        let result = Solution::intersection(input.nums);
        println!("{:?}", result);
    }
}
