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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum: i32 = 0;
        Self::dfs(root.clone(), &mut sum);
        root
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = node {
            Self::some(node, sum)
        }
    }

    fn some(node: Rc<RefCell<TreeNode>>, sum: &mut i32) {
        Self::dfs(node.borrow().right.clone(), sum);
        node.borrow_mut().val += *sum;
        *sum = node.borrow().val;
        Self::dfs(node.borrow().left.clone(), sum);
    }
}

struct Input {
    num1: String,
    num2: String,
}

fn main() {
    let inputs = [Input {
        num1: "1+1i".to_string(),
        num2: "1+1i".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::complex_number_multiply(input.num1, input.num2);
        println!("{:?}", result);
    }
}
