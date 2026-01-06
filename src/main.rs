struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        for i in 1..(mountain.len() - 1) {
            let left = mountain[i - 1];
            let middle = mountain[i];
            let right = mountain[i + 1];

            if (left < middle) && (middle > right) {
                result.push(i as i32);
            }
        }

        result
    }
}

struct Input {
    mountain: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            mountain: [2, 4, 4].to_vec(),
        },
        Input {
            mountain: [1, 4, 3, 8, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_peaks(input.mountain);
        println!("{:?}", result);
    }
}
