use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeType = Option<Rc<RefCell<TreeNode>>>;

fn newnode(val: i32, left: TreeType, right: TreeType) -> TreeType {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn newval(val: i32) -> TreeType {
    newnode(val, None, None)
}

fn newleft(val: i32, left: TreeType) -> TreeType {
    newnode(val, left, None)
}

fn newright(val: i32, right: TreeType) -> TreeType {
    newnode(val, None, right)
}

struct Solution {}
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        }

        let mut stack = vec![Rc::clone(&root.unwrap())];
        let mut visiteds = HashSet::<*mut TreeNode>::new();
        let mut result = Vec::<String>::new();
        while !stack.is_empty() {
            let node_ref = if let Some(n) = stack.last() {
                Rc::clone(n)
            } else {
                break;
            };

            if visiteds.contains(&node_ref.as_ptr()) {
                stack.pop();
                result.push(")".to_string());
                continue;
            }

            visiteds.insert(node_ref.as_ptr());
            result.push("(".to_string());

            let node = node_ref.borrow();
            result.push(node.val.to_string());

            if node.left.is_none() && node.right.is_some() {
                result.push("()".to_string());
            }

            if let Some(r) = &node.right {
                stack.push(Rc::clone(r));
            }

            if let Some(l) = &node.left {
                stack.push(Rc::clone(l));
            }
        }

        let size = result.len();
        String::from_iter(result.into_iter().skip(1).take(size - 2))
    }
}
struct Input {
    root: TreeType,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newnode(1, newleft(2, newval(4)), newval(3)),
        },
        Input {
            root: newnode(1, newright(2, newval(4)), newval(3)),
        },
        Input {
            root: newleft(-1, newleft(-2, newleft(-3, newval(-4)))),
        },
    ];

    for input in inputs.iter() {
        let root = input.root.clone();
        let result = Solution::tree2str(root);
        println!("{result}");
    }
}
