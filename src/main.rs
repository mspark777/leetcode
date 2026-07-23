struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let an = a.len();
        let bn = b.len();
        let x = bn / an + 2;

        a.repeat(x)
            .find(&b)
            .map(|i| ((i + bn - 1) / an) + 1)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}

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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        root.map(|root| {
            let val = root.borrow().val;
            Self::dfs(&mut max, Some(root), val)
        });
        max
    }

    fn dfs(max: &mut i32, node: Option<Rc<RefCell<TreeNode>>>, x: i32) -> i32 {
        node.map_or(0, |node| {
            let node = node.borrow();

            let l = Self::dfs(max, node.left.clone(), node.val);
            let r = Self::dfs(max, node.right.clone(), node.val);
            *max = (*max).max(l + r);

            match x == node.val {
                true => 1 + l.max(r),
                _ => 0,
            }
        })
    }
}

struct Input {
    a: String,
    b: String,
}

fn main() {
    let inputs = [Input {
        a: "abcd".to_string(),
        b: "cdabcdab".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::repeated_string_match(input.a, input.b);
        println!("{:?}", result);
    }
}
