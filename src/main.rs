use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeType = Option<Rc<RefCell<TreeNode>>>;

fn newnode(val: i32, left: TreeType, right: TreeType) -> TreeType {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn newval(val: i32) -> TreeType {
    newnode(val, None, None)
}

fn newleft(val: i32, left: TreeType) -> TreeType {
    newnode(val, left, None)
}

fn newright(val: i32, right: TreeType) -> TreeType {
    newnode(val, None, right)
}

fn preorder(node: &TreeType) {
    if let Some(n) = node {
        let bo = n.borrow();
        preorder(&bo.left);
        print!("{} ", bo.val);
        preorder(&bo.right);
    }
}

struct Solution {}
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::contains_one(&root) {
            root
        } else {
            None
        }
    }

    fn contains_one(ntype: &TreeType) -> bool {
        if let Some(noderef) = ntype {
            let mut node = noderef.borrow_mut();
            let left_contained = Self::contains_one(&node.left);
            if !left_contained {
                node.left = None;
            }

            let right_contained = Self::contains_one(&node.right);
            if !right_contained {
                node.right = None;
            }

            (node.val == 1) || left_contained || right_contained
        } else {
            false
        }
    }
}

struct Input {
    root: TreeType,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newright(1, newnode(0, newval(0), newval(1))),
        },
        Input {
            root: newnode(
                1,
                newnode(0, newval(0), newval(0)),
                newnode(1, newval(0), newval(1)),
            ),
        },
        Input {
            root: newnode(
                1,
                newnode(1, newleft(1, newval(0)), newval(1)),
                newnode(0, newval(0), newval(1)),
            ),
        },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::prune_tree(root);
        preorder(&result);
        println!("");
    }
}
