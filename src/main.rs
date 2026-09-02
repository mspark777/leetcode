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
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sufficient_subset(
        node: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        node.as_ref()?;
        let val = node.as_ref()?.borrow().val;
        let mut left = node.as_ref()?.borrow_mut().left.take();
        let mut right = node.as_ref()?.borrow_mut().right.take();
        if left.is_none() && right.is_none() {
            if val < limit {
                return None;
            } else {
                return node;
            }
        }
        if left.is_some() {
            left = Self::sufficient_subset(left, limit - val);
        }
        if right.is_some() {
            right = Self::sufficient_subset(right, limit - val);
        }
        if left.is_none() && right.is_none() {
            return None;
        }
        node.as_ref()?.borrow_mut().left = left;
        node.as_ref()?.borrow_mut().right = right;
        node
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
