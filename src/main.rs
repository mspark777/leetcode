struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut boxes = capacity;
        boxes.sort_unstable();

        let mut apple_count = apple.iter().sum::<i32>();
        let mut result = 0;

        for b in boxes.iter().rev().copied() {
            apple_count -= b;
            result += 1;
            if apple_count < 1 {
                break;
            }
        }

        result
    }
}

struct Input {
    apple: Vec<i32>,
    capacity: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            apple: [1, 3, 2].to_vec(),
            capacity: [4, 3, 1, 5, 2].to_vec(),
        },
        Input {
            apple: [5, 5, 5].to_vec(),
            capacity: [2, 4, 2, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_boxes(input.apple, input.capacity);
        println!("{:?}", result);
    }
}
