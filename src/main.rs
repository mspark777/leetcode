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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn generate(
        node_cnt: i32,
        memo: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(node) = memo.get(&node_cnt) {
            return node.clone();
        }

        let mut result = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();

        for left_node_cnt in 0..node_cnt {
            let right_node_cnt = node_cnt - left_node_cnt - 1;

            let left_tree = Self::generate(left_node_cnt, memo);
            let right_tree = Self::generate(right_node_cnt, memo);

            for st1 in left_tree.iter() {
                for st2 in right_tree.iter() {
                    let curr_node = Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: st1.clone(),
                        right: st2.clone(),
                    })));
                    result.push(curr_node);
                }
            }
        }
        memo.insert(node_cnt, result.clone());
        result
    }

    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut memo = HashMap::new();
        memo.insert(0, vec![]);
        memo.insert(1, vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]);

        Self::generate(n, &mut memo)
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [Input {
        words: ["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]
            .map(|v| v.to_string())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::num_special_equiv_groups(input.words);
        println!("{:?}", result);
    }
}
