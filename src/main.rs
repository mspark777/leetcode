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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::recurse(root.clone(), target_sum as i64, vec![].as_mut())
    }

    pub fn recurse(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        vec: &mut Vec<i64>,
    ) -> i32 {
        let node = match root {
            Some(node) => node,
            _ => return 0,
        };

        let node = node.borrow();
        let mut count = 0;
        vec.push(0);
        for x in vec.iter_mut() {
            *x += node.val as i64;
            if *x == target_sum {
                count += 1;
            }
        }
        count = count
            + Self::recurse(node.left.clone(), target_sum, vec)
            + Self::recurse(node.right.clone(), target_sum, vec);
        vec.pop();
        for x in vec.iter_mut() {
            *x -= node.val as i64
        }
        count
    }
}

struct Input {
    intervals: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            intervals: [[1, 2]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            intervals: [[3, 4], [2, 3], [1, 2]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            intervals: [[1, 4], [2, 3], [3, 4]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_right_interval(input.intervals);
        println!("{:?}", result);
    }
}
