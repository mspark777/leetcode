struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();

        let mut even = 0usize;
        let mut odd = 1usize;
        let n = nums.len();

        while (even < n) && (odd < n) {
            if result[even] & 1 == 0 {
                even += 2;
            } else if result[odd] & 1 == 1 {
                odd += 2;
            } else {
                let t0 = result[even];
                let t1 = result[odd];
                result[even] = t1;
                result[odd] = t0;
                even += 2;
                odd += 2;
            }
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
            nums: vec![4, 2, 5, 7],
        },
        Input { nums: vec![2, 3] },
    ];

    for input in inputs {
        let result = Solution::sort_array_by_parity_ii(input.nums);
        println!("{result:?}");
    }
}
