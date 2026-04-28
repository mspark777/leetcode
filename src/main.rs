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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let amount = Self::travel(root);
        amount.0.max(amount.1)
    }

    fn travel(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(n) => Self::travel1(n.clone()),
            _ => (0, 0),
        }
    }

    fn travel1(node: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let left = Self::travel(node.borrow().left.clone());
        let right = Self::travel(node.borrow().right.clone());

        (
            left.0.max(left.1) + right.0.max(right.1),
            node.borrow().val + left.0 + right.0,
        )
    }
}

struct Input {
    preorder: String,
}

fn main() {
    let inputs = [Input {
        preorder: "9,#,92,#,#".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::is_valid_serialization(input.preorder);
        println!("{:?}", result);
    }
}
