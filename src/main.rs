struct Solution {}

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut odd = Vec::<i32>::with_capacity(nums.len() / 2);
        let mut even = Vec::<i32>::with_capacity(nums.len() / 2 + (nums.len() & 1));
        for (i, n) in nums.iter().cloned().enumerate() {
            if i & 1 == 1 {
                odd.push(n);
            } else {
                even.push(n);
            }
        }

        even.sort_unstable();
        odd.sort_unstable_by_key(|n| -n);
        let mut result = Vec::<i32>::with_capacity(nums.len());
        for (e, o) in even.iter().cloned().zip(odd.iter().cloned()) {
            result.push(e);
            result.push(o);
        }

        if nums.len() & 1 == 1 {
            result.push(*even.last().unwrap());
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
            nums: [4, 1, 2, 3].to_vec(),
        },
        Input {
            nums: [2, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sort_even_odd(input.nums);
        println!("{:?}", result);
    }
}
