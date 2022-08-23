use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self { val, left, right })))
    }
}

struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root.clone()];

        while let Some(top) = stack.pop() {
            if let Some(node) = top {
                let TreeNode { left, right, .. } = &mut *node.borrow_mut();
                mem::swap(left, right);
                stack.push(left.clone());
                stack.push(right.clone());
            }
        }

        root
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl Input {
    fn travel(node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            Self::travel(n.borrow().left.clone());
            print!("{} ", n.borrow().val);
            Self::travel(n.borrow().right.clone());
        }
    }
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: TreeNode::new(
                4,
                TreeNode::new(
                    2,
                    TreeNode::new(1, None, None),
                    TreeNode::new(3, None, None),
                ),
                TreeNode::new(
                    7,
                    TreeNode::new(6, None, None),
                    TreeNode::new(9, None, None),
                ),
            ),
        },
        Input {
            root: TreeNode::new(
                2,
                TreeNode::new(1, None, None),
                TreeNode::new(3, None, None),
            ),
        },
        Input { root: None },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::invert_tree(root);
        Input::travel(result);
        println!("")
    }
}
