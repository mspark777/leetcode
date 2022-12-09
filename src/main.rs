use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn newnode(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    return Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
}

fn newleft(val: i32, left: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, left, None);
}

fn newright(val: i32, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, right);
}

fn newval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, None);
}

struct Solution {}
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.as_ref();
        let val = node.unwrap().borrow().val;
        return Self::travel(node, val, val);
    }

    fn travel(node: Option<&Rc<RefCell<TreeNode>>>, curmax: i32, curmin: i32) -> i32 {
        if node.is_none() {
            return curmax - curmin;
        }

        let rnode = node.unwrap().borrow();
        let curmax = curmax.max(rnode.val);
        let curmin = curmin.min(rnode.val);

        let left = Self::travel(rnode.left.as_ref(), curmax, curmin);
        let right = Self::travel(rnode.right.as_ref(), curmax, curmin);
        return left.max(right);
    }
}

fn main() {
    let inputs = [
        newnode(
            8,
            newnode(3, newval(1), newnode(6, newval(4), newval(7))),
            newright(10, newleft(14, newval(13))),
        ),
        newright(1, newright(2, newleft(0, newval(3)))),
    ];

    for root in inputs {
        let result = Solution::max_ancestor_diff(root);
        println!("{result}");
    }
}
