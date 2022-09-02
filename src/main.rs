#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if let Some(root_node) = root.as_ref() {
            let mut result = Vec::<f64>::new();

            let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
            queue.push_back(Rc::clone(root_node));
            while !queue.is_empty() {
                let size = queue.len();
                let mut total = 0f64;
                for _ in 0..size {
                    let cloned = Rc::clone(queue.pop_front().as_ref().unwrap());
                    let head = cloned.borrow();
                    total += head.val as f64;

                    if let Some(l) = head.left.as_ref() {
                        queue.push_back(Rc::clone(l));
                    }

                    if let Some(r) = head.right.as_ref() {
                        queue.push_back(Rc::clone(r));
                    }
                }

                result.push(total / (size as f64));
            }

            result
        } else {
            Vec::<f64>::new()
        }
    }
}

struct Input {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn newnode(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn newval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    newnode(val, None, None)
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newnode(3, newval(9), newnode(20, newval(15), newval(7))),
        },
        Input {
            root: newnode(3, newnode(9, newval(15), newval(7)), newval(20)),
        },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::average_of_levels(root);
        println!("{result:?}");
    }
}
