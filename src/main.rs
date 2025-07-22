struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let mut min_diff = i32::MAX;
        let mut result = Vec::<Vec<i32>>::new();
        for (left, right) in arr.iter().cloned().zip(arr.iter().skip(1).cloned()) {
            let diff = right - left;
            if diff < min_diff {
                min_diff = diff;
                result.clear();
                result.push(vec![left, right]);
            } else if diff == min_diff {
                result.push(vec![left, right]);
            }
        }

        result
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: [4, 2, 1, 3].to_vec(),
        },
        Input {
            arr: [1, 3, 6, 10, 15].to_vec(),
        },
        Input {
            arr: [3, 8, -10, 23, 19, -4, -14, 27].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_abs_difference(input.arr);
        println!("{:?}", result);
    }
}
