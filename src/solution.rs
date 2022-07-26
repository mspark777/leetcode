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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (root, p, q) = match (root, p, q) {
            (Some(r), Some(p), Some(q)) => (r, p, q),
            _ => {
                return None;
            }
        };

        if Rc::ptr_eq(&root, &p) || Rc::ptr_eq(&root, &q) {
            return Some(root.clone());
        }

        let rb = (*root).borrow();
        let left = Self::lowest_common_ancestor(rb.left.clone(), Some(p.clone()), Some(q.clone()));
        let right =
            Self::lowest_common_ancestor(rb.right.clone(), Some(p.clone()), Some(q.clone()));

        if left.is_none() {
            right.clone()
        } else if right.is_none() {
            left.clone()
        } else {
            Some(root.clone())
        }
    }
}
