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

fn treetoarr(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
    if let Some(nn) = node {
        let bo = nn.borrow();
        nums.push(bo.val);
        treetoarr(&bo.left, nums);
        treetoarr(&bo.right, nums);
    }
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        let mut stack = vec![(Rc::clone(root.as_ref().unwrap()), 2)];

        let mut targets = Vec::<Rc<RefCell<TreeNode>>>::new();
        while let Some(top) = stack.pop() {
            let pos = top.1;
            if pos > depth {
                continue;
            }

            let node = &top.0;
            if pos == depth {
                targets.push(Rc::clone(node));
            }

            let bo = node.borrow();
            if let Some(left) = &bo.left {
                stack.push((Rc::clone(left), pos + 1));
            }

            if let Some(right) = &bo.right {
                stack.push((Rc::clone(right), pos + 1));
            }
        }

        for target in targets.iter_mut() {
            let mut bo = target.borrow_mut();
            let left = bo.left.take();
            let right = bo.right.take();
            bo.left = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right: None,
            })));
            bo.right = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right,
            })));
        }

        return root;
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
    depth: i32,
}

fn main() {
    let inputs = vec![
        Input {
            root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
            val: 1,
            depth: 2,
        },
        Input {
            root: newleft(4, newnode(2, newval(3), newval(1))),
            val: 1,
            depth: 3,
        },
        Input {
            root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
            val: 1,
            depth: 1,
        },
    ];

    for Input { root, val, depth } in inputs.into_iter() {
        let result = Solution::add_one_row(root, val, depth);
        let mut nums = Vec::<i32>::new();
        treetoarr(&result, &mut nums);
        println!("{nums:?}");
    }
}
