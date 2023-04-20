mod utils;

use utils::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(some_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if some_root.is_none() {
            return 0;
        }

        let root = some_root.as_ref().unwrap();
        let mut result = 1;
        let mut queue = vec![(Rc::clone(root), 0)];

        while !queue.is_empty() {
            let start = queue.first().unwrap().1;
            let end = queue.last().unwrap().1;
            result = result.max(end - start + 1);

            let mut children = Vec::<(Rc<RefCell<TreeNode>>, i32)>::new();
            for qnode in queue.iter() {
                let node = qnode.0.borrow();
                let idx = qnode.1 - start;
                let some_left = &node.left;
                let some_right = &node.right;

                if let Some(left) = some_left {
                    children.push((Rc::clone(left), 2 * idx + 1));
                }

                if let Some(right) = some_right {
                    children.push((Rc::clone(right), 2 * (idx + 1)));
                }
            }

            queue = children;
        }

        return result;
    }
}

fn main() {
    let inputs = [
        TreeNode::new_node(
            1,
            TreeNode::new_node(3, TreeNode::new_val(5), TreeNode::new_val(3)),
            TreeNode::new_right(2, TreeNode::new_val(9)),
        ),
        TreeNode::new_node(
            1,
            TreeNode::new_left(3, TreeNode::new_left(5, TreeNode::new_val(6))),
            TreeNode::new_right(2, TreeNode::new_left(9, TreeNode::new_val(7))),
        ),
        TreeNode::new_node(
            1,
            TreeNode::new_left(3, TreeNode::new_val(5)),
            TreeNode::new_val(2),
        ),
    ];

    for root in inputs {
        let result = Solution::width_of_binary_tree(root);
        println!("{result}");
    }
}
