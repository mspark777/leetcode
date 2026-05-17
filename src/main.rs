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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(r) => {
                if r.borrow().val.cmp(&key).is_eq() {
                    let mut r = r.borrow_mut();
                    if r.left.as_ref().is_none() && r.right.as_ref().is_none() {
                        return None;
                    } else if r.left.as_ref().is_some() && r.right.as_ref().is_none() {
                        return r.left.take();
                    } else if r.left.as_ref().is_none() && r.right.as_ref().is_some() {
                        return r.right.take();
                    } else {
                        let mini = Self::min(r.right.clone()).unwrap().borrow().val;
                        r.val = mini;
                        r.right = Self::delete_node(r.right.clone(), mini);
                    }
                } else if r.borrow().val.cmp(&key).is_gt() {
                    let mut t = r.borrow_mut();
                    t.left = Self::delete_node(t.left.clone(), key);
                } else {
                    let mut t = r.borrow_mut();
                    t.right = Self::delete_node(t.right.clone(), key)
                }
                Some(r)
            }
            None => None,
        }
    }
    fn min(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut temp = None;
        while root.as_ref().is_some() {
            temp = root.clone();
            root = root.unwrap().borrow_mut().left.clone();
        }
        temp.clone()
    }
}

struct Input {
    points: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            points: [[0, 0], [1, 0], [2, 0]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            points: [[1, 1], [2, 2], [3, 3]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            points: [[1, 1]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::number_of_boomerangs(input.points);
        println!("{:?}", result);
    }
}
