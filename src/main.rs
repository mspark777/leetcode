struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_element = 0;

        for (i, &n) in arr.iter().enumerate() {
            max_element = max_element.max(n);
            if max_element == (i as i32) {
                result += 1;
            }
        }

        return result;
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![4, 3, 2, 1, 0],
        },
        Input {
            arr: vec![1, 0, 2, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::max_chunks_to_sorted(input.arr);
        println!("{result}");
    }
}
