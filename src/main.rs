use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn newnode(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    return Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
}

fn newleft(val: i32, left: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, left, None);
}

fn newright(val: i32, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, right);
}

fn newval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    return newnode(val, None, None);
}

struct Solution {}
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack1 = Vec::<i32>::new();
        let mut stack2 = Vec::<i32>::new();

        Self::dfs(&mut stack1, root1.as_ref());
        Self::dfs(&mut stack2, root2.as_ref());

        return stack1 == stack2;
    }

    fn dfs(stack: &mut Vec<i32>, node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let r = n.borrow();
            let val = r.val;
            let left = r.left.as_ref();
            let right = r.right.as_ref();

            if left.is_none() && right.is_none() {
                stack.push(val);
            }

            Self::dfs(stack, left);
            Self::dfs(stack, right);
        }
    }
}

struct Input {
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    let inputs = [
        Input {
            root1: newnode(
                3,
                newnode(5, newval(6), newnode(2, newval(7), newval(4))),
                newnode(1, newval(9), newval(8)),
            ),
            root2: newnode(
                3,
                newnode(5, newval(6), newval(7)),
                newnode(1, newval(4), newnode(2, newval(9), newval(8))),
            ),
        },
        Input {
            root1: newnode(1, newval(2), newval(3)),
            root2: newnode(1, newval(3), newval(2)),
        },
    ];

    for Input { root1, root2 } in inputs {
        let result = Solution::leaf_similar(root1, root2);
        println!("{result}");
    }
}
