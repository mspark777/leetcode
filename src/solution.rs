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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        Self::preorder(&root, &mut result);

        result
    }

    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let v = n.borrow();
            result.push(v.val);
            Self::preorder(&v.left, result);
            Self::preorder(&v.right, result);
        }
    }
}
