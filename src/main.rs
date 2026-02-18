struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut result = s.chars().collect::<Vec<char>>();
        let n = result.len();
        for i in 1..n {
            let left = result[i - 1] as u8;
            let right = result[i] as u8;

            if ((left & 1) == (right & 1)) && (left > right) {
                result[i - 1] = right as char;
                result[i] = left as char;
                break;
            }
        }

        result.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "45320".to_string(),
        },
        Input {
            s: "001".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::get_smallest_string(input.s);
        println!("{:?}", result);
    }
}
