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

struct StackNode {
    node: Rc<RefCell<TreeNode>>,
    target: i32,
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut stack = vec![StackNode {
            node: Rc::clone(&root.unwrap()),
            target: target_sum,
        }];

        while let Some(top) = stack.pop() {
            let node = top.node.borrow();
            let target = top.target;
            let newval = target - node.val;
            let mut isleaf = true;

            if let Some(left) = &node.left {
                isleaf = false;
                stack.push(StackNode {
                    node: Rc::clone(left),
                    target: newval,
                });
            }

            if let Some(right) = &node.right {
                isleaf = false;
                stack.push(StackNode {
                    node: Rc::clone(right),
                    target: newval,
                });
            }

            if isleaf && (newval == 0) {
                return true;
            }
        }

        return false;
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
}

fn main() {
    let inputs = vec![
        Input {
            root: newnode(
                5,
                newleft(4, newnode(11, newval(7), newval(2))),
                newnode(8, newval(13), newright(4, newval(1))),
            ),
            target_sum: 22,
        },
        Input {
            root: newnode(5, newval(2), newval(3)),
            target_sum: 5,
        },
        Input {
            root: None,
            target_sum: 0,
        },
    ];

    for Input { root, target_sum } in inputs.into_iter() {
        let result = Solution::has_path_sum(root, target_sum);
        println!("{result:?}");
    }
}
