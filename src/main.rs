struct Solution {}

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut even_queue = std::collections::BinaryHeap::new();
        let mut odd_queue = std::collections::BinaryHeap::new();
        let nums = num
            .to_string()
            .chars()
            .map(|ch| (ch as i32) - (b'0' as i32))
            .collect::<Vec<_>>();

        for n in nums.iter().cloned() {
            if n & 1 == 1 {
                odd_queue.push(n);
            } else {
                even_queue.push(n);
            }
        }

        let mut result = 0;
        for n in nums.iter().cloned() {
            result *= 10;
            if n & 1 == 1 {
                result += odd_queue.pop().unwrap();
            } else {
                result += even_queue.pop().unwrap();
            }
        }

        result
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 1234 }, Input { num: 65875 }];

    for input in inputs {
        let result = Solution::largest_integer(input.num);
        println!("{:?}", result);
    }
}
