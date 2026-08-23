struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut curr_max = nums[0];
        let mut possible_max = nums[0];
        let mut result = 1;

        for (i, num) in nums.into_iter().enumerate().skip(1) {
            if num < curr_max {
                result = (i + 1) as i32;
                curr_max = possible_max;
            } else {
                possible_max = possible_max.max(num);
            }
        }

        result
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

struct CBTInserter {
    nodes: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while !queue.is_empty() {
            if let Some(Some(node)) = queue.pop_front() {
                nodes.push(Some(node.clone()));
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
        }
        Self { nodes }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let n = self.nodes.len();
        let node = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        self.nodes.push(node.clone());
        if n.is_multiple_of(2) {
            self.nodes[(n - 1) / 2].as_ref().unwrap().borrow_mut().right = node;
        } else {
            self.nodes[(n - 1) / 2].as_ref().unwrap().borrow_mut().left = node;
        }
        self.nodes[(n - 1) / 2].as_ref().unwrap().borrow().val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.nodes[0].clone()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [5, 0, 3, 8, 6].to_vec(),
    }];

    for input in inputs {
        let result = Solution::partition_disjoint(input.nums);
        println!("{:?}", result);
    }
}
