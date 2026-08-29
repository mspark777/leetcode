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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn new_node(val: i32, left: TypeNode, right: TypeNode) -> TypeNode {
        let mut tnode = TreeNode::new(val);
        tnode.left = left;
        tnode.right = right;
        Some(Rc::new(RefCell::new(tnode)))
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> TypeNode {
        Self::dfs(&preorder[..])
    }

    fn dfs(p: &[i32]) -> TypeNode {
        if p.len() == 0 {
            return None;
        }

        if p.len() == 1 {
            return Self::new_node(p[0], None, None);
        }
        let j = match p[1..].binary_search(&p[0]) {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        };

        let left = Self::dfs(&p[1..j]);
        let right = Self::dfs(&p[j..]);
        Self::new_node(p[0], left, right)
    }
}

struct Input {
    start_value: i32,
    target: i32,
}

fn main() {
    let inputs = [Input {
        start_value: 2,
        target: 3,
    }];

    for input in inputs {
        let result = Solution::broken_calc(input.start_value, input.target);
        println!("{:?}", result);
    }
}
