mod solution;

use std::{cell::RefCell, rc::Rc};

use solution::{Solution, TreeNode};

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
    pv: i32,
    qv: i32,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
}

fn find_node(root: Option<Rc<RefCell<TreeNode>>>, v: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = root {
        let br = r.borrow();
        if br.val == v {
            return Some(r.clone());
        }

        let left = find_node(br.left.clone(), v);
        if left.is_some() {
            return left;
        }

        find_node(br.right.clone(), v)
    } else {
        None
    }
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
    let mut inputs: Vec<Input> = vec![
        Input {
            root: create_node(
                3,
                create_node(
                    5,
                    create_node(6, None, None),
                    create_node(2, create_node(7, None, None), create_node(4, None, None)),
                ),
                create_node(1, create_node(0, None, None), create_node(8, None, None)),
            ),
            p: None,
            q: None,
            pv: 5,
            qv: 1,
        },
        Input {
            root: create_node(
                3,
                create_node(
                    5,
                    create_node(6, None, None),
                    create_node(2, create_node(7, None, None), create_node(4, None, None)),
                ),
                create_node(1, create_node(0, None, None), create_node(8, None, None)),
            ),
            p: None,
            q: None,
            pv: 5,
            qv: 4,
        },
        Input {
            root: create_node(1, create_node(2, None, None), None),
            p: None,
            q: None,
            pv: 1,
            qv: 2,
        },
    ];

    for input in inputs.iter_mut() {
        input.q = find_node(input.root.clone(), input.qv);
        input.p = find_node(input.root.clone(), input.pv);
        let result =
            Solution::lowest_common_ancestor(input.root.clone(), input.p.clone(), input.q.clone());
        let result: Option<i32> = if let Some(r) = result {
            Some(r.borrow().val)
        } else {
            None
        };

        println!("{result:?}");
    }
}
