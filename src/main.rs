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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }
    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(n) => {
                let n_ref = n.borrow();
                let (r_deepth, r_ref) = Self::dfs(&n_ref.left);
                let (l_deepth, l_ref) = Self::dfs(&n_ref.right);
                if r_deepth > l_deepth {
                    (r_deepth + 1, r_ref)
                } else if r_deepth < l_deepth {
                    (l_deepth + 1, l_ref)
                } else {
                    (l_deepth + 1, node.clone())
                }
            }
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "()".to_string(),
    }];

    for input in inputs {
        let result = Solution::score_of_parentheses(input.s);
        println!("{:?}", result);
    }
}
