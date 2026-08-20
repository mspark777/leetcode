struct Solution;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut a = a;

        a.sort_unstable();

        let mut b = b.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
        b.sort_by_key(|a| a.1);

        let mut result = vec![0; n];
        let mut slow = 0;
        let mut fast = n - 1;
        while let Some((opponent_idx, opponent)) = b.pop() {
            result[opponent_idx] = match opponent >= a[fast] {
                true => {
                    let v = a[slow];
                    slow += 1;
                    v
                }
                _ => {
                    let v = a[fast];
                    fast -= match fast > 0 {
                        true => 1,
                        _ => 0,
                    };
                    v
                }
            }
        }
        result
    }
}

struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums1: [2, 7, 11, 15].to_vec(),
        nums2: [1, 10, 4, 11].to_vec(),
    }];

    for input in inputs {
        let result = Solution::advantage_count(input.nums1, input.nums2);
        println!("{:?}", result);
    }
}
