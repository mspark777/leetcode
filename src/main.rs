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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut result = 0;
        let mut stack = vec![Rc::clone(root.as_ref().unwrap())];

        while !stack.is_empty() {
            if let Some(top) = stack.pop() {
                let node = top.borrow();
                let val = node.val;
                let left = node.left.as_ref();
                let right = node.right.as_ref();

                if (low <= val) && (val <= high) {
                    result += val;
                }

                if low < val {
                    if let Some(l) = left {
                        stack.push(Rc::clone(l));
                    }
                }

                if val < high {
                    if let Some(r) = right {
                        stack.push(Rc::clone(r));
                    }
                }
            }
        }

        return result;
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    low: i32,
    high: i32,
}

fn main() {
    let inputs = [
        Input {
            low: 7,
            high: 15,
            root: newnode(
                10,
                newnode(5, newval(3), newval(7)),
                newright(15, newval(18)),
            ),
        },
        Input {
            low: 6,
            high: 10,
            root: newnode(
                10,
                newnode(5, newleft(3, newval(1)), newleft(7, newval(6))),
                newnode(5, newval(13), newval(18)),
            ),
        },
    ];

    for Input { root, low, high } in inputs {
        let result = Solution::range_sum_bst(root, low, high);
        println!("{result}");
    }
}
