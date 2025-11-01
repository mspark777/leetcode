struct Solution {}

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let time = time.chars().collect::<Vec<char>>();
        let mut result = 1;

        if time[4] == '?' {
            result *= 10
        }

        if time[3] == '?' {
            result *= 6
        }

        if (time[0] == '?') && (time[1] == '?') {
            result *= 24
        } else {
            if time[1] == '?' {
                if time[0] == '2' {
                    result *= 4
                } else {
                    result *= 10
                }
            }

            if time[0] == '?' {
                if time[1] < '4' {
                    result *= 3
                } else {
                    result *= 2
                }
            }
        }

        result
    }
}

struct Input {
    time: String,
}

fn main() {
    let inputs = [
        Input {
            time: "?5:00".to_string(),
        },
        Input {
            time: "0?:0?".to_string(),
        },
        Input {
            time: "??:??".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_time(input.time);
        println!("{:?}", result);
    }
}
