use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type OTreeNode = Option<Rc<RefCell<TreeNode>>>;

fn newnode(val: i32, left: OTreeNode, right: OTreeNode) -> OTreeNode {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn newright(val: i32, right: OTreeNode) -> OTreeNode {
    newnode(val, None, right)
}

fn newval(val: i32) -> OTreeNode {
    newnode(val, None, None)
}

struct Solution {}
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut result = 0;
        let mut stack = vec![(Rc::clone(&root.unwrap()), 0)];

        while let Some(top) = stack.pop() {
            let (noderef, path) = top;
            let node = noderef.borrow();
            let new_path = path ^ (1 << node.val);

            let mut is_leaf = true;
            if let Some(left) = &node.left {
                is_leaf = false;
                stack.push((Rc::clone(left), new_path));
            }

            if let Some(right) = &node.right {
                is_leaf = false;
                stack.push((Rc::clone(right), new_path));
            }

            if is_leaf {
                if (new_path & (new_path - 1)) == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}

struct Input {
    root: OTreeNode,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            root: newnode(2, newnode(3, newval(3), newval(1)), newright(1, newval(1))),
        },
        Input {
            root: newnode(2, newnode(1, newval(1), newright(3, newval(1))), newval(1)),
        },
        Input { root: newval(9) },
    ];

    for Input { root } in inputs.into_iter() {
        let result = Solution::pseudo_palindromic_paths(root);
        println!("{result}");
    }
}
