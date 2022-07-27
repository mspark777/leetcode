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

fn treetoarr(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut node = node;
    let mut nums = Vec::<i32>::new();
    while let Some(n) = node {
        nums.push(n.borrow().val);
        node = n.borrow().right.clone();
    }

    nums
}

fn main() {
    let mut inputs: Vec<Input> = vec![
        Input {
            root: create_node(
                1,
                create_node(2, create_node(3, None, None), create_node(4, None, None)),
                create_node(5, None, create_node(8, None, None)),
            ),
        },
        Input { root: None },
        Input {
            root: create_node(0, None, None),
        },
    ];

    for input in inputs.iter_mut() {
        Solution::flatten(&mut input.root);

        println!("{:?}", treetoarr(input.root.clone()));
    }
}
