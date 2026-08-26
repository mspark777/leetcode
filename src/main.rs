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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::_flip_equiv(root1, root2).unwrap_or(false)
    }

    fn _flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<bool> {
        if root1.is_none() && root2.is_none() {
            return Some(true);
        }
        if root1.is_none() || root2.is_none() {
            return Some(false);
        }
        if root1.as_ref()?.borrow().val != root2.as_ref()?.borrow().val {
            return Some(false);
        }
        let left1 = root1.as_ref()?.borrow().left.clone();
        let right1 = root1.as_ref()?.borrow().right.clone();
        let left2 = root2.as_ref()?.borrow().left.clone();
        let right2 = root2.as_ref()?.borrow().right.clone();
        let v = (Self::_flip_equiv(left1.clone(), left2.clone())?
            && Self::_flip_equiv(right1.clone(), right2.clone())?)
            || (Self::_flip_equiv(left1, right2)? && Self::_flip_equiv(right1, left2)?);
        Some(v)
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [1, 2, 3, 4].to_vec(),
    }];

    for input in inputs {
        let result = Solution::largest_time_from_digits(input.arr);
        println!("{:?}", result);
    }
}
