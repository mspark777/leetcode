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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count = k;
        let mut result = 0;

        Self::inorder(root.as_ref(), &mut count, &mut result);
        result
    }

    fn inorder(node: Option<&Rc<RefCell<TreeNode>>>, count: &mut i32, result: &mut i32) {
        let node = match node {
            Some(n) => n,
            _ => return,
        };

        if *count == 0 {
            return;
        }

        Self::inorder(node.borrow().left.as_ref(), count, result);
        *count -= 1;
        if *count == 0 {
            *result = node.borrow().val;
        }

        Self::inorder(node.borrow().right.as_ref(), count, result);
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "3+2*2".to_string(),
        },
        Input {
            s: " 3+5 / 2 ".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::calculate(input.s);
        println!("{:?}", result);
    }
}
