struct Solution {}

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::<i32>::with_capacity(arr.len());
        for &num in arr.iter() {
            let twice = num * 2;
            if seen.contains(&twice) {
                return true;
            }

            if num & 1 == 1 {
                seen.insert(num);
                continue;
            }

            let half = num / 2;
            if seen.contains(&half) {
                return true;
            }

            seen.insert(num);
        }

        return false;
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![10, 2, 5, 3],
        },
        Input {
            arr: vec![3, 1, 7, 11],
        },
    ];

    for input in inputs {
        let result = Solution::check_if_exist(input.arr);
        println!("{result}");
    }
}
