use std::cell::RefCell;
use std::collections::HashSet;
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

fn newright(val: i32, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, right);
}

fn newval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, None);
}

struct Solution {}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut stack = vec![Rc::clone(root.as_ref().unwrap())];
        let mut memo = HashSet::<i32>::new();

        while let Some(topref) = stack.pop() {
            let top = topref.borrow();
            let val = top.val;
            let target = k - val;
            if memo.contains(&target) {
                return true;
            }

            memo.insert(val);

            if let Some(left) = &top.left {
                stack.push(Rc::clone(left));
            }

            if let Some(right) = &top.right {
                stack.push(Rc::clone(right));
            }
        }

        return false;
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
            k: 9,
        },
        Input {
            root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
            k: 28,
        },
    ];

    for Input { root, k } in inputs {
        let result = Solution::find_target(root, k);
        println!("{result}");
    }
}
