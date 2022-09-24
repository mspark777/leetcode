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
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn newleft(val: i32, left: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    newnode(val, left, None)
}

fn newval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    newnode(val, None, None)
}

struct StackNode {
    path: Vec<i32>,
    node: Rc<RefCell<TreeNode>>,
    sum: i32,
}

struct Solution {}
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::<Vec<i32>>::new();
        let mut stack = vec![StackNode {
            path: vec![],
            node: Rc::clone(&root.unwrap()),
            sum: 0,
        }];

        while let Some(top) = &stack.pop() {
            let path = &top.path;
            let node = top.node.borrow();
            let val = node.val;
            let sum = top.sum + val;
            let mut is_leaf = true;

            if let Some(left) = &node.left {
                is_leaf = false;
                let mut newpath = path.clone();
                newpath.push(val);
                stack.push(StackNode {
                    path: newpath,
                    node: Rc::clone(left),
                    sum,
                });
            }

            if let Some(right) = &node.right {
                is_leaf = false;
                let mut newpath = path.clone();
                newpath.push(val);
                stack.push(StackNode {
                    path: newpath,
                    node: Rc::clone(right),
                    sum,
                });
            }

            if is_leaf && (sum == target_sum) {
                let mut newpath = path.clone();
                newpath.push(val);
                result.push(newpath);
            }
        }

        result
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newnode(
                5,
                newleft(4, newnode(11, newval(7), newval(2))),
                newnode(8, newval(13), newnode(4, newval(5), newval(1))),
            ),
            target_sum: 22,
        },
        Input {
            root: newnode(1, newval(2), newval(3)),
            target_sum: 5,
        },
        Input {
            root: newleft(1, newval(2)),
            target_sum: 0,
        },
    ];

    for Input { root, target_sum } in inputs.into_iter() {
        let result = Solution::path_sum(root, target_sum);
        println!("{result:?}");
    }
}
