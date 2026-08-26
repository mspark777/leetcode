struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs.len();
        let n = strs.first().map_or(0, |s| s.len());

        let mut inorder = 0u128;
        let mut count = 0;

        for col in 0..n {
            let mut new_inorder = 0u128;

            for row in 1..m {
                if (inorder & (1 << row)) != 0 {
                    continue;
                }

                let prev = strs[row - 1].as_bytes();
                let curr = strs[row].as_bytes();

                if prev[col] < curr[col] {
                    new_inorder |= 1 << row;
                } else if prev[col] > curr[col] {
                    count += 1;
                    new_inorder = 0;
                    break;
                }
            }
            inorder |= new_inorder;
        }
        count
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [1, 2, 3, 4].to_vec(),
    }];

    for input in inputs {
        let result = Solution::can_reorder_doubled(input.arr);
        println!("{:?}", result);
    }
}
