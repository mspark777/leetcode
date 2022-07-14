mod solution;

use solution::Solution;

struct Input {
    preorder: Vec<i32>,
    inorder: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            preorder: vec![3, 9, 20, 15, 7],
            inorder: vec![9, 3, 15, 20, 7],
        },
        Input {
            preorder: vec![-1],
            inorder: vec![-1],
        },
    ];

    for input in inputs {
        let result = Solution::build_tree(input.preorder, input.inorder);
        println!("{result:?}");
    }
}
