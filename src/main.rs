use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }

    pub fn new2(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new(val, left, right))))
    }
}

struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth2(&root)
    }

    pub fn min_depth2(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let b = n.borrow();
            match (&b.left, &b.right) {
                (Some(_), Some(_)) => Self::min_depth2(&b.left).min(Self::min_depth2(&b.right)) + 1,
                _ => Self::min_depth2(&b.left).max(Self::min_depth2(&b.right)) + 1,
            }
        } else {
            0
        }
    }
}

fn main() {
    let inputs = [
        TreeNode::new2(
            3,
            TreeNode::new2(9, None, None),
            TreeNode::new2(
                20,
                TreeNode::new2(15, None, None),
                TreeNode::new2(7, None, None),
            ),
        ),
        TreeNode::new2(
            2,
            None,
            TreeNode::new2(
                3,
                None,
                TreeNode::new2(
                    4,
                    None,
                    TreeNode::new2(5, None, TreeNode::new2(6, None, None)),
                ),
            ),
        ),
    ];

    for input in inputs {
        let result = Solution::min_depth(input);
        println!("{result:?}");
    }
}
