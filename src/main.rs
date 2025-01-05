struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let forward = 1;
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut diff_arr = vec![0; n];

        for shift in shifts.iter() {
            let start = shift[0];
            let end = shift[1];
            let direction = shift[2];
            if direction == forward {
                diff_arr[start as usize] += 1;
                if ((end + 1) as usize) < n {
                    diff_arr[(end + 1) as usize] -= 1;
                }
            } else {
                diff_arr[start as usize] -= 1;
                if ((end + 1) as usize) < n {
                    diff_arr[(end + 1) as usize] += 1;
                }
            }
        }

        let mut result = vec![' '; n];
        let mut number_of_shifts = 0i32;
        for (i, ch) in s.iter().cloned().enumerate() {
            number_of_shifts = (number_of_shifts + diff_arr[i]) % 26;
            if number_of_shifts < 0 {
                number_of_shifts += 26;
            }

            let code = ('a' as i32) + ((ch as i32) - ('a' as i32) + number_of_shifts) % 26;
            result[i] = code as u8 as char;
        }

        return result.iter().collect();
    }
}

struct Input {
    s: &'static str,
    shifts: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            s: "abc",
            shifts: vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]],
        },
        Input {
            s: "dztz",
            shifts: vec![vec![0, 0, 0], vec![1, 1, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::shifting_letters(input.s.to_string(), input.shifts);
        println!("{result:?}");
    }
}
