struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;
        let mut imax = 0i64;
        let mut dmax = 0i64;

        for num in nums.iter().cloned() {
            let num = num as i64;
            result = result.max(dmax * num);
            dmax = dmax.max(imax - num);
            imax = imax.max(num);
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
            nums: vec![12, 6, 1, 2, 7],
        },
        Input {
            nums: vec![1, 10, 3, 4, 19],
        },
        Input {
            nums: vec![1, 2, 3],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_triplet_value(input.nums);
        println!("{result:?}");
    }
}
