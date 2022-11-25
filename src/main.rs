struct Solution {}
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack = Vec::<i32>::new();
        let mut dp = vec![0; arr.len()];

        for (i, n) in arr.iter().cloned().enumerate() {
            while let Some(top) = stack.last() {
                if arr[*top as usize] >= n {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(top) = stack.last() {
                let idx = i as i32;
                dp[i] = dp[*top as usize] + (idx - top) * n;
            } else {
                let idx = i as i32;
                dp[i] = (idx + 1) * n;
            }

            stack.push(i as i32);
        }

        let mut result = 0;
        for count in dp {
            result += count;
            result %= 1000000007;
        }

        return result;
    }
}

fn main() {
    let inputs = [vec![3, 1, 2, 4], vec![11, 81, 94, 43, 3]];

    for arr in inputs {
        let result = Solution::sum_subarray_mins(arr);
        println!("{result}");
    }
}
