#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(r) = root {
            let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
            queue.push_back(r);
            let mut result = Vec::<Vec<i32>>::new();

            let mut depth = 0;
            while !queue.is_empty() {
                depth += 1;
                let count = queue.len();
                let mut level = Vec::<i32>::with_capacity(depth);
                for _ in 0..count {
                    if let Some(n) = queue.pop_front() {
                        let node = n.borrow();
                        level.push(node.val);

                        if let Some(left) = &node.left {
                            queue.push_back(left.clone());
                        }

                        if let Some(right) = &node.right {
                            queue.push_back(right.clone());
                        }
                    }
                }

                if !level.is_empty() {
                    result.push(level);
                }
            }

            result
        } else {
            Vec::new()
        }
    }
}
