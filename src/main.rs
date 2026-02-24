struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        for i in 1..height.len() {
            if height[i - 1] > threshold {
                result.push(i as i32);
            }
        }

        result
    }
}

struct Input {
    height: Vec<i32>,
    threshold: i32,
}

fn main() {
    let inputs = [
        Input {
            height: [1, 2, 3, 4, 5].to_vec(),
            threshold: 2,
        },
        Input {
            height: [10, 1, 10, 1, 10].to_vec(),
            threshold: 3,
        },
        Input {
            height: [10, 1, 10, 1, 10].to_vec(),
            threshold: 10,
        },
    ];

    for input in inputs {
        let result = Solution::stable_mountains(input.height, input.threshold);
        println!("{:?}", result);
    }
}
