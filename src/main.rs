struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let slen = s.len();
        if slen != goal.len() {
            return false;
        }

        let doubled_len = slen * 2;
        let doubled_string = format!("{s}{s}");
        if let Some(i) = doubled_string.find(&goal) {
            return i < doubled_len;
        }

        return false;
    }
}

struct Input {
    s: String,
    goal: String,
}

fn main() {
    let inputs = vec![
        Input {
            s: String::from("abcde"),
            goal: String::from("cdeab"),
        },
        Input {
            s: String::from("abcde"),
            goal: String::from("abced"),
        },
    ];

    for input in inputs {
        let result = Solution::rotate_string(input.s, input.goal);
        println!("{result}");
    }
}
