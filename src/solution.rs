#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut result = Vec::<i32>::new();

        if let Some(r) = root {
            stack.push(r.clone());
        } else {
            return result;
        }

        while let Some(top) = stack.pop() {
            let node = top.borrow();
            result.push(node.val);

            if let Some(left) = &node.left {
                stack.push(left.clone());
            }

            if let Some(right) = &node.right {
                stack.push(right.clone());
            }
        }

        result.reverse();
        result
    }
}
