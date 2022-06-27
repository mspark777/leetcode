pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(n) => {
                let node = n.borrow();
                let new_sum = target_sum - node.val;
                match (&node.left, &node.right) {
                    (None, None) => node.val == target_sum,
                    (Some(left), None) => Self::has_path_sum(Some(left.clone()), new_sum),
                    (None, Some(right)) => Self::has_path_sum(Some(right.clone()), new_sum),
                    (Some(left), Some(right)) => {
                        Self::has_path_sum(Some(left.clone()), new_sum)
                            || Self::has_path_sum(Some(right.clone()), new_sum)
                    }
                }
            }
        }
    }
}
