use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let root = root.unwrap();
        let mut result: Vec<i32> = vec![];
        let mut q = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        q.push_back(root.clone());

        while !q.is_empty() {
            let qlen = q.len();
            for i in 1..=qlen {
                if let Some(node) = q.pop_front() {
                    let n = node.borrow_mut();
                    if i == qlen {
                        result.push(n.val);
                    }

                    if let Some(l) = &n.left {
                        q.push_back(l.clone());
                    }

                    if let Some(r) = &n.right {
                        q.push_back(r.clone());
                    }
                }
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

    pub fn create_tree_from_vec(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() {
            return None;
        }

        let mut q = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        let mut i = 0;
        let root = Self::new_optional(arr[i]);
        q.push_back(root.clone());
        i += 1;
        while !q.is_empty() && (i < arr.len()) {
            if let Some(t1) = q.pop_front() {
                if let Some(t) = t1 {
                    let mut node = t.borrow_mut();
                    node.left = Self::new_optional(arr[i]);
                    q.push_back(node.left.clone());
                    i += 1;
                    if i >= arr.len() {
                        break;
                    }

                    node.right = Self::new_optional(arr[i]);
                    q.push_back(node.right.clone());
                    i += 1;
                }
            }
        }

        root
    }

    fn new_optional(val: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(v) = val {
            Some(Rc::new(RefCell::new(Self::new(v))))
        } else {
            None
        }
    }
}
