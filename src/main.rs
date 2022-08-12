#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::q_or_p_is_none(&p, &q) {
            return None;
        }

        let pval = Self::get_node_val(&p.unwrap());
        let qval = Self::get_node_val(&q.unwrap());

        let mut curnode = root.clone();
        while let Some(cur) = curnode.clone() {
            let val = Self::get_node_val(&cur);
            if (pval < val) && (qval < val) {
                curnode = cur.borrow().left.clone();
            } else if (pval > val) && (qval > val) {
                curnode = cur.borrow().right.clone();
            } else {
                break;
            }
        }

        curnode
    }

    #[inline]
    fn q_or_p_is_none(
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(_), Some(_)) => false,
            _ => true,
        }
    }

    #[inline]
    fn get_node_val(n: &Rc<RefCell<TreeNode>>) -> i32 {
        n.borrow().val
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    let inputs: Vec<Input> = vec![];

    for input in inputs {
        let root = input.root;
        let result = Solution::lowest_common_ancestor(root, None, None);
        println!("{:?}", result);
    }
}
