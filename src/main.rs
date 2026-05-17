struct Solution;

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                format!(
                    "{},{},{}",
                    node.borrow().val,
                    Self::serialize(self, node.borrow().left.clone()),
                    Self::serialize(self, node.borrow().right.clone())
                )
            }
            _ => String::new(),
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let pre_order = data
            .split(",")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        let mut in_order = pre_order.clone();
        in_order.sort();

        match pre_order.is_empty() {
            true => None,
            _ => Self::mk(&pre_order, &in_order, 0, 0, in_order.len()),
        }
    }

    fn mk(
        pre_order: &[i32],
        in_order: &[i32],
        i: usize,
        l: usize,
        r: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = TreeNode::new(pre_order[i]);
        let j = in_order.binary_search(&pre_order[i]).unwrap();
        if j > l {
            tree.left = Self::mk(pre_order, in_order, i + 1, l, j);
        }

        if j < (r - 1) {
            tree.right = Self::mk(pre_order, in_order, i + 1 + j - l, j + 1, r);
        }

        Some(Rc::new(RefCell::new(tree)))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

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
