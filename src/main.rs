struct Solution {}
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number;
        const BASE: i32 = 26;

        let mut result = Vec::<u8>::new();

        while n > 0 {
            n -= 1;

            let temp = b'A' + ((n % BASE) as u8);
            result.push(temp);

            n /= BASE;
        }

        result.reverse();
        String::from_utf8(result).expect("No")
    }
}

struct Input {
    column_number: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { column_number: 1 },
        Input { column_number: 28 },
        Input { column_number: 701 },
    ];

    for input in inputs {
        let result = Solution::convert_to_title(input.column_number);
        println!("{:?}", result);
    }
}
