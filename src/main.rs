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

struct Solution;

impl Solution {
    pub fn solve(
        root: Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, i32>,
        max_freq: &mut i32,
    ) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut sum = 0;
        let root = root.as_ref().unwrap().borrow();
        sum += root.val;
        sum += Solution::solve(root.left.clone(), map, max_freq);
        sum += Solution::solve(root.right.clone(), map, max_freq);

        *map.entry(sum).or_insert(0) += 1;

        if let Some(freq) = map.get(&sum) {
            *max_freq = std::cmp::max(*freq, *max_freq);
        }

        sum
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        let mut max_freq = 0;
        Solution::solve(root.clone(), &mut map, &mut max_freq);

        let mut result = Vec::<i32>::new();
        for (key, val) in map {
            if val == max_freq {
                result.push(key);
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 3].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::next_greater_elements(input.nums);
        println!("{:?}", result);
    }
}
