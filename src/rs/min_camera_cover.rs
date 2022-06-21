use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
enum Status {
    LEAF,
    CAMERA,
    NOCAMERA,
}

#[allow(dead_code)]
struct DFS {
    depth: i32,
}

#[allow(dead_code)]
impl DFS {
    fn travel(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) -> Status {
        if let Some(node) = root {
            self.travel2(node.clone())
        } else {
            Status::NOCAMERA
        }
    }

    fn travel2(&mut self, node: Rc<RefCell<TreeNode>>) -> Status {
        let left = self.travel(&node.borrow().left);
        let right = self.travel(&node.borrow().right);
        match (left, right) {
            (Status::LEAF, _) | (_, Status::LEAF) => {
                self.depth += 1;
                Status::CAMERA
            }
            (Status::CAMERA, _) | (_, Status::CAMERA) => Status::NOCAMERA,
            _ => Status::LEAF,
        }
    }
}

#[allow(dead_code)]
pub struct MinCameraCover {}

#[allow(dead_code)]
impl MinCameraCover {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut dfs = DFS { depth: 0 };
        let status = dfs.travel(&root);
        let depth = dfs.depth;
        match status {
            Status::LEAF => depth + 1,
            _ => depth,
        }
    }
}
