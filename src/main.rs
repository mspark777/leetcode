struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut begin = 0usize;
        if target < nums[begin] {
            return 0;
        }

        let mut end = nums.len();
        if target > nums[end - 1] {
            return end as i32;
        }

        while begin < end {
            let middle = (begin + end) / 2;
            let num = nums[middle];

            if num < target {
                begin = middle + 1;
            } else if num > target {
                end = middle;
            } else {
                return middle as i32;
            }
        }

        return begin as i32;
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 3, 5, 6],
            target: 5,
        },
        Input {
            nums: vec![1, 3, 5, 6],
            target: 2,
        },
        Input {
            nums: vec![1, 3, 5, 6],
            target: 7,
        },
    ];

    for Input { nums, target } in inputs {
        let result = Solution::search_insert(nums, target);
        println!("{result}");
    }
}
