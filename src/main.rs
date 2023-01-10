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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        return Self::is_same_tree2(p.as_ref(), q.as_ref());
    }

    pub fn is_same_tree2(
        p: Option<&Rc<RefCell<TreeNode>>>,
        q: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }

        let pp = p.unwrap().borrow();
        let qq = q.unwrap().borrow();

        if pp.val != qq.val {
            return false;
        }

        return Self::is_same_tree2(pp.left.as_ref(), qq.left.as_ref())
            && Self::is_same_tree2(pp.right.as_ref(), qq.right.as_ref());
    }
}

fn main() {
    let inputs = [
        (
            newnode(1, newval(2), newval(3)),
            newnode(1, newval(2), newval(3)),
        ),
        (newleft(1, newval(2)), newright(1, newval(2))),
        (
            newnode(1, newval(2), newval(1)),
            newnode(1, newval(1), newval(2)),
        ),
    ];

    for (p, q) in inputs {
        let result = Solution::is_same_tree(q, p);
        println!("{result}");
    }
}
