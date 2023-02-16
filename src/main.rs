/*
leetcode
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeType = Option<Rc<RefCell<TreeNode>>>;

fn newnode(val: i32, left: TreeType, right: TreeType) -> TreeType {
    return Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
}

fn newright(val: i32, right: TreeType) -> TreeType {
    return Some(Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right,
    })));
}

fn newval(val: i32) -> TreeType {
    return Some(Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
    })));
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn max_depth(root: TreeType) -> i32 {
        let mut result = 0;
        Self::travel(&root, 0, &mut result);

        return result;
    }

    fn travel(node: &TreeType, depth: i32, max_depth: &mut i32) {
        if let Some(n) = node {
            let d = depth + 1;
            Self::travel(&n.borrow().left, d, max_depth);
            Self::travel(&n.borrow().right, d, max_depth);
        } else {
            *max_depth = depth.max(*max_depth);
        }
    }
}

fn main() {
    let inputs: Vec<TreeType> = vec![
        newnode(3, newval(9), newnode(20, newval(15), newval(7))),
        newright(1, newval(2)),
    ];

    for root in inputs {
        let result = Solution::max_depth(root);
        println!("{result}");
    }
}
