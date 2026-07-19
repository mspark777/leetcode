struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_idx, val)| val)
        {
            None => None,
            Some((idx, val)) => Self::build(&nums[..idx], &nums[idx + 1..], val),
        }
    }

    fn build(left: &[i32], right: &[i32], val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let left = match left
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_idx, val)| val)
        {
            None => None,
            Some((idx, val)) => Self::build(&left[..idx], &left[idx + 1..], val),
        };

        let right = match right
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_idx, val)| val)
        {
            None => None,
            Some((idx, val)) => Self::build(&right[..idx], &right[idx + 1..], val),
        };

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "abc".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::count_substrings(input.s);
        println!("{:?}", result);
    }
}
