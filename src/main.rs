struct Solution;

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();

        for idx in (0..n - 1).rev() {
            if arr[idx] > arr[idx + 1] {
                let mut right = idx + 1;
                let mut swap_idx = idx + 1;

                while right < n && arr[right] < arr[idx] {
                    if arr[right] != arr[swap_idx] {
                        swap_idx = right;
                    }
                    right += 1;
                }

                arr.swap(swap_idx, idx);
                break;
            }
        }

        arr
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
