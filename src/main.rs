struct Solution {}
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        const FACTOR: u8 = b'A' - 1;
        let mut result = 0;

        for code in column_title.as_bytes() {
            let diff = (code - FACTOR) as i32;
            result = result * 26 + diff;
        }

        result
    }
}

struct Input {
    column_title: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { column_title: "A" },
        Input { column_title: "AB" },
        Input { column_title: "ZY" },
    ];

    for input in inputs {
        let column_title = input.column_title.to_string();
        let result = Solution::title_to_number(column_title);
        println!("{:?}", result);
    }
}
