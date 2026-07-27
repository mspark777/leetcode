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

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Solution::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Solution::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node
                }
                r
            }
        })
    }
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index as usize >= self.size {
            return -1;
        }

        let mut current = &self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            }
        }

        if let Some(node) = current {
            node.val
        } else {
            -1
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(Node::new(val));

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index as usize > self.size {
            return;
        }

        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut new_node = Box::new(Node::new(val));
        let mut current = &mut self.head;

        for _ in 0..(index - 1) {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        if let Some(node) = current {
            new_node.next = node.next.take();
            node.next = Some(new_node);
        }
        self.size += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index as usize >= self.size {
            return;
        }

        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
            self.size -= 1;
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..(index - 1) {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        if let Some(node) = current {
            if let Some(next_node) = &mut node.next {
                node.next = next_node.next.take();
            }
        }
        self.size -= 1;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [Input {
        nums: [4, 3, 2, 3, 5, 2, 1].to_vec(),
        k: 4,
    }];

    for input in inputs.into_iter() {
        let result = Solution::can_partition_k_subsets(input.nums, input.k);
        println!("{:?}", result);
    }
}
