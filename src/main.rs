struct Solution {}

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        s.chars()
            .collect::<Vec<char>>()
            .chunks(k)
            .map(|chunk| {
                if chunk.len() >= k {
                    chunk.iter().collect()
                } else {
                    [chunk.to_vec(), vec![fill; k - chunk.len()]]
                        .concat()
                        .iter()
                        .collect()
                }
            })
            .collect()
    }
}

struct Input {
    s: &'static str,
    k: i32,
    fill: char,
}

fn main() {
    let inputs = vec![
        Input {
            s: "abcdefghi",
            k: 3,
            fill: 'x',
        },
        Input {
            s: "abcdefghij",
            k: 3,
            fill: 'x',
        },
    ];

    for input in inputs {
        let result = Solution::divide_string(input.s.to_string(), input.k, input.fill);
        println!("{:?}", result);
    }
}
