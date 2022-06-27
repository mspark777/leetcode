mod solution;

use solution::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
}

fn create_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode {
        val,
        left: left.clone(),
        right: right.clone(),
    })))
}

fn main() {
    let inputs = [
        Input {
            target_sum: 22,
            root: create_node(
                5,
                create_node(
                    4,
                    create_node(11, create_node(7, None, None), create_node(2, None, None)),
                    None,
                ),
                create_node(
                    8,
                    create_node(13, None, None),
                    create_node(4, None, create_node(1, None, None)),
                ),
            ),
        },
        Input {
            target_sum: 5,
            root: create_node(1, create_node(2, None, None), create_node(1, None, None)),
        },
        Input {
            target_sum: 0,
            root: None,
        },
    ];

    for input in inputs {
        let result = Solution::has_path_sum(input.root, input.target_sum);
        println!("{result:?}");
    }
}
