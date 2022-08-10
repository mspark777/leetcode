#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::travel(&nums, 0, nums.len())
    }

    fn travel(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l >= r {
            return None;
        }

        let mid = (l + r) / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::travel(nums, l, mid),
            right: Self::travel(nums, mid + 1, r),
        })))
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![-10, -3, 0, 5, 9],
        },
        Input { nums: vec![1, 3] },
    ];

    for input in inputs {
        let nums = input.nums;
        let result = Solution::sorted_array_to_bst(nums);
        println!("{:?}", result);
    }
}
