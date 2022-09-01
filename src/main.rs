use std::cell::RefCell;
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
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

struct StackNode {
    node: Rc<RefCell<TreeNode>>,
    max: i32,
}

impl StackNode {
    #[inline]
    fn new(node: Rc<RefCell<TreeNode>>, max: i32) -> Self {
        StackNode {
            node: node.clone(),
            max,
        }
    }
}

struct Solution {}
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_node) = root {
            let mut result = 0;
            let mut stack = vec![StackNode::new(
                Rc::clone(&root_node),
                root_node.borrow().val,
            )];

            while let Some(stack_node) = stack.pop() {
                let node = stack_node.node.borrow();
                let val = node.val;
                let max = stack_node.max.max(val);

                if max == val {
                    result += 1;
                }

                if let Some(left) = node.left.as_ref() {
                    stack.push(StackNode::new(Rc::clone(left), max));
                }

                if let Some(right) = node.right.as_ref() {
                    stack.push(StackNode::new(Rc::clone(right), max));
                }
            }

            result
        } else {
            0
        }
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: TreeNode::new(
                3,
                TreeNode::new(1, TreeNode::new(3, None, None), None),
                TreeNode::new(
                    4,
                    TreeNode::new(1, None, None),
                    TreeNode::new(5, None, None),
                ),
            ),
        },
        Input {
            root: TreeNode::new(
                3,
                TreeNode::new(
                    3,
                    TreeNode::new(4, None, None),
                    TreeNode::new(2, None, None),
                ),
                None,
            ),
        },
        Input {
            root: TreeNode::new(1, None, None),
        },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::good_nodes(root);
        println!("{result}");
    }
}
