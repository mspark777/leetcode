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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut node: Option<Rc<RefCell<TreeNode>>> = root.clone();

        while let Some(n) = node {
            let mut n = n.borrow_mut();
            if let Some(mut cur) = n.left.clone() {
                loop {
                    let right = cur.borrow().right.clone();
                    if let Some(r) = right {
                        cur = r;
                    } else {
                        break;
                    }
                }

                let mut b = cur.borrow_mut();
                b.right = n.right.take();
                n.right = n.left.take();
            }

            node = n.right.clone();
        }
    }
}
