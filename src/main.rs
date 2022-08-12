struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let nlen = nums.len();
        if nlen < 2 {
            return;
        }

        let mut i = nlen - 2;
        while nums[i] >= nums[i + 1] {
            if i > 0 {
                i -= 1
            } else {
                return Self::reverse(nums, 0);
            }
        }

        let mut j = nlen - 1;
        while nums[i] >= nums[j] {
            j -= 1;
        }

        Self::swap(nums, i, j);
        Self::reverse(nums, i + 1)
    }

    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }

    fn reverse(nums: &mut Vec<i32>, start: usize) {
        let mut i = start;
        let mut j = nums.len() - 1;

        while i < j {
            Self::swap(nums, i, j);
            i += 1;
            j -= 1;
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3],
        },
        Input {
            nums: vec![3, 2, 1],
        },
        Input {
            nums: vec![1, 1, 5],
        },
    ];

    for input in inputs {
        let mut nums = input.nums;
        Solution::next_permutation(&mut nums);
        println!("{:?}", nums);
    }
}
