struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len() as i32;
        let mut left = 0;
        let mut right = n - 1;
        let mut result = 0;

        while left <= right {
            let mid = (left + right) / 2;
            if (n - mid) <= citations[mid as usize] {
                result = n - mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        result
    }
}

struct Input {
    citations: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            citations: [0, 1, 3, 5, 6].to_vec(),
        },
        Input {
            citations: [1, 2, 100].to_vec(),
        },
        Input {
            citations: [1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::h_index(input.citations);
        println!("{:?}", result);
    }
}
