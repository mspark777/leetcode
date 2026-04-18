struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut citations = citations;
        citations.sort();

        citations
            .into_iter()
            .enumerate()
            .find(|&(i, c)| {
                let count = (n - i) as i32;
                c >= count
            })
            .map(|v| (n - v.0) as i32)
            .unwrap_or_default()
    }
}

struct Input {
    citations: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            citations: [3, 0, 6, 1, 5].to_vec(),
        },
        Input {
            citations: [1, 3, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::h_index(input.citations);
        println!("{:?}", result);
    }
}
