struct Solution {}
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut left = 0usize;
        let mut right = arr.len() - k;

        while left < right {
            let mid = (left + right) / 2;
            let a = arr[mid + k] - x;
            let b = x - arr[mid];
            if a < b {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let range = left..left + k;
        return arr[range].to_vec();
    }
}

struct Input {
    arr: Vec<i32>,
    k: i32,
    x: i32,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![1, 2, 3, 4, 5],
            k: 4,
            x: 3,
        },
        Input {
            arr: vec![1, 2, 3, 4, 5],
            k: 4,
            x: -1,
        },
    ];

    for Input { arr, k, x } in inputs.into_iter() {
        let result = Solution::find_closest_elements(arr, k, x);
        println!("{result:?}");
    }
}
