mod solution;

use solution::Solution;
use solution::TreeNode;

struct Input {
    root: Vec<Option<i32>>,
}

fn main() {
    let inputs = [
        Input {
            root: vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)],
        },
        Input {
            root: vec![Some(1), None, Some(3)],
        },
        Input { root: vec![] },
    ];

    for input in inputs {
        let result = Solution::right_side_view(TreeNode::create_tree_from_vec(input.root));
        println!("{result:?}");
    }
}
