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

pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder;
        let mut inorder = inorder;

        preorder.reverse();
        inorder.reverse();
        Self::build(&mut preorder, &mut inorder, None)
    }

    fn build(
        preorder: &mut Vec<i32>,
        inorder: &mut Vec<i32>,
        bound: Option<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || (bound.is_some() && (*inorder.last().unwrap() == bound.unwrap())) {
            return None;
        }

        let mut node = TreeNode::new(preorder.pop().unwrap());
        node.left = Self::build(preorder, inorder, Some(node.val));

        inorder.pop();
        node.right = Self::build(preorder, inorder, bound);

        Some(Rc::new(RefCell::new(node)))
    }
}
