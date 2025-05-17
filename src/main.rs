struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0usize;
        let mut mid = 0usize;
        let mut high = nums.len() - 1;

        while mid <= high {
            if nums[mid] == 0 {
                let temp = nums[low];
                nums[low] = nums[mid];
                nums[mid] = temp;
                low += 1;
                mid += 1;
            } else if nums[mid] == 1 {
                mid += 1;
            } else {
                let temp = nums[mid];
                nums[mid] = nums[high];
                nums[high] = temp;
                if high > 1 {
                    high -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 0, 2, 1, 1, 0],
        },
        Input {
            nums: vec![2, 0, 1],
        },
        Input { nums: vec![2] },
    ];

    for mut input in inputs {
        Solution::sort_colors(&mut input.nums);
        println!("{:?}", input.nums);
    }
}
