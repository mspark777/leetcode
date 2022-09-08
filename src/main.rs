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

struct Solution {}
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut stack = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut result = Vec::<i32>::new();
        let mut top = Some(Rc::clone(&root.unwrap()));

        while top.is_some() || !stack.is_empty() {
            while let Some(node_rc) = top {
                stack.push(Rc::clone(&node_rc));
                let node_bo = node_rc.borrow();
                top = if let Some(left_rc) = &node_bo.left {
                    Some(Rc::clone(left_rc))
                } else {
                    None
                };
            }

            let top_node = stack.pop().unwrap();
            let top_bo = top_node.borrow();
            result.push(top_bo.val);
            top = if let Some(right_rc) = &top_bo.right {
                Some(Rc::clone(right_rc))
            } else {
                None
            };
        }

        result
    }
}
struct Input {
    root: TreeType,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newright(1, newleft(2, newval(3))),
        },
        Input { root: None },
        Input { root: newval(1) },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::inorder_traversal(root);
        println!("{result:?}");
    }
}
