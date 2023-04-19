mod utils;

use utils::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_steps = 0;
        Self::dfs(&root, true, 0, &mut max_steps);
        Self::dfs(&root, false, 0, &mut max_steps);

        return max_steps;
    }

    fn dfs(some_node: &Option<Rc<RefCell<TreeNode>>>, left: bool, steps: i32, max_steps: &mut i32) {
        if some_node.is_none() {
            return;
        }

        let node = some_node.as_ref().unwrap().borrow();
        let max = *max_steps;
        *max_steps = max.max(steps);
        if left {
            Self::dfs(&node.left, false, steps + 1, max_steps);
            Self::dfs(&node.right, true, 1, max_steps);
        } else {
            Self::dfs(&node.left, false, 1, max_steps);
            Self::dfs(&node.right, true, steps + 1, max_steps);
        }
    }
}

fn main() {
    let inputs = [
        TreeNode::new_right(
            1,
            TreeNode::new_node(
                1,
                TreeNode::new_val(1),
                TreeNode::new_node(
                    1,
                    TreeNode::new_right(1, TreeNode::new_right(1, TreeNode::new_val(1))),
                    TreeNode::new_val(1),
                ),
            ),
        ),
        TreeNode::new_node(
            1,
            TreeNode::new_right(
                1,
                TreeNode::new_node(
                    1,
                    TreeNode::new_right(1, TreeNode::new_val(1)),
                    TreeNode::new_val(1),
                ),
            ),
            TreeNode::new_val(1),
        ),
        TreeNode::new_val(1),
    ];

    for root in inputs {
        let result = Solution::longest_zig_zag(root);
        println!("{result}");
    }
}
