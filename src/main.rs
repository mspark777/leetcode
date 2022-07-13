mod solution;

use std::{cell::RefCell, rc::Rc};

use solution::{Solution, TreeNode};

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn create_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, right, left })))
}

fn main() {
    let inputs = [
        Input {
            root: create_node(
                3,
                create_node(9, None, None),
                create_node(20, create_node(15, None, None), create_node(7, None, None)),
            ),
        },
        Input {
            root: create_node(1, None, None),
        },
        Input { root: None },
    ];

    for input in inputs {
        let result = Solution::level_order(input.root);
        println!("{result:?}");
    }
}
