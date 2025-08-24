struct Solution {}

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let len = arr.len();
        let skip = len / 20;
        let count = len - 2 * skip;

        let sum = arr.iter().skip(skip).take(count).sum::<i32>();
        sum as f64 / count as f64
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            arr: [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3].to_vec(),
        },
        Input {
            arr: [6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0].to_vec(),
        },
        Input {
            arr: [
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
            ]
            .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::trim_mean(input.arr);
        println!("{:?}", result);
    }
}
