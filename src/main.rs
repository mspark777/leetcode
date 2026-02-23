struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .map(|n| format!("{:b}", n))
            .collect::<Vec<String>>()
            .join("-")
    }
}

struct Input {
    date: String,
}

fn main() {
    let inputs = [
        Input {
            date: "2080-02-29".to_string(),
        },
        Input {
            date: "1900-01-01".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::convert_date_to_binary(input.date);
        println!("{:?}", result);
    }
}
