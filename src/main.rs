struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let last_row = num_rows - 1;
        let mut row = 0usize;
        let mut down = true;
        let mut result = vec![Vec::<char>::new(); num_rows];

        for ch in s.chars() {
            result[row].push(ch);
            if row == last_row {
                down = false;
            } else if row == 0 {
                down = true;
            }

            if down {
                row += 1
            } else {
                row -= 1
            }
        }

        return result
            .iter()
            .map(|r| {
                let s: String = r.iter().collect();
                return s;
            })
            .collect();
    }
}

fn main() {
    let inputs = [
        ("PAYPALISHIRING", 3),
        ("PAYPALISHIRING", 4),
        ("A", 1),
        ("AB", 1),
    ];

    for (s, num_rows) in inputs {
        let result = Solution::convert(s.to_string(), num_rows);
        println!("{result}");
    }
}
