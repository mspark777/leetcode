struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut nodes = 1;
        for node in preorder.split(',').map(|s| s.as_bytes()) {
            if node[0] == b',' {
                continue;
            }

            nodes -= 1;

            if nodes < 0 {
                return false;
            }

            if node[0] != b'#' {
                nodes += 2
            }
        }

        nodes == 0
    }
}

struct Input {
    preorder: String,
}

fn main() {
    let inputs = [Input {
        preorder: "9,#,92,#,#".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::is_valid_serialization(input.preorder);
        println!("{:?}", result);
    }
}
