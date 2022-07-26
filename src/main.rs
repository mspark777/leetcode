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
    Some(Rc::new(RefCell::new(TreeNode {
        val,
        left: left.clone(),
        right: right.clone(),
    })))
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: create_node(1, None, create_node(2, create_node(3, None, None), None)),
        },
        Input { root: None },
        Input {
            root: create_node(1, None, None),
        },
    ];

    for input in inputs {
        let result = Solution::preorder_traversal(input.root);

        println!("{result:?}");
    }
}
