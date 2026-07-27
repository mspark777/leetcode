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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Solution::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Solution::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node
                }
                r
            }
        })
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [Input {
        nums: [4, 3, 2, 3, 5, 2, 1].to_vec(),
        k: 4,
    }];

    for input in inputs.into_iter() {
        let result = Solution::can_partition_k_subsets(input.nums, input.k);
        println!("{:?}", result);
    }
}
