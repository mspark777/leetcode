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
    pub fn print_tree(root: TypeNode) -> Vec<Vec<String>> {
        let height = Self::dfs(root.clone());
        let n = 2usize.pow(height) - 1;
        let mut tree = vec![vec![String::new(); n]; height as usize];
        Self::print(root.clone(), &mut tree, height, 0, 0, true);
        tree
    }

    fn dfs(root: TypeNode) -> u32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + Self::dfs(node.left.clone()).max(Self::dfs(node.right.clone()))
            }
        }
    }

    fn print(
        root: TypeNode,
        tree: &mut Vec<Vec<String>>,
        h: u32,
        r: u32,
        pc: u32,
        left_child: bool,
    ) {
        match root {
            None => {}
            Some(node) => {
                let node = node.borrow();
                let c = if r == 0 {
                    2u32.pow(h - 1) - 1
                } else if left_child {
                    pc - 2u32.pow(h - r - 1)
                } else {
                    pc + 2u32.pow(h - r - 1)
                };
                tree[r as usize][c as usize] = format!("{}", node.val);
                Self::print(node.left.clone(), tree, h, r + 1, c, true);
                Self::print(node.right.clone(), tree, h, r + 1, c, false);
            }
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "abc".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::count_substrings(input.s);
        println!("{:?}", result);
    }
}
