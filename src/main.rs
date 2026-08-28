struct Solution;

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
use std::rc::Rc;
impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![-1],
        };

        let mut res = vec![];
        let mut stack = vec![root];

        for cur in 0..voyage.len() {
            let node = stack.pop().unwrap();
            if node.borrow().val != voyage[cur] {
                return vec![-1];
            }
            match (&node.borrow().left, &node.borrow().right) {
                (Some(l), Some(r)) => {
                    if l.borrow().val == voyage[cur + 1] {
                        stack.push(r.clone());
                        stack.push(l.clone());
                    } else {
                        stack.push(l.clone());
                        stack.push(r.clone());
                        res.push(node.borrow().val);
                    }
                }
                (Some(a), _) | (_, Some(a)) => {
                    stack.push(a.clone());
                }
                _ => (),
            };
        }
        res
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
