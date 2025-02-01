struct Solution {}

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut np = 0usize;
        let mut tp = 0usize;

        let name = name.chars().collect::<Vec<char>>();
        let typed = typed.chars().collect::<Vec<char>>();

        while (np < name.len()) && (tp < typed.len()) {
            if name[np] == typed[tp] {
                np += 1;
                tp += 1;
            } else if (tp > 0) && typed[tp] == typed[tp - 1] {
                tp += 1;
            } else {
                return false;
            }
        }

        if np != name.len() {
            return false;
        }

        while tp < typed.len() {
            if typed[tp] != typed[tp - 1] {
                return false;
            } else {
                tp += 1;
            }
        }

        return true;
    }
}

struct Input {
    name: &'static str,
    typed: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            name: "alex",
            typed: "aaleex",
        },
        Input {
            name: "saeed",
            typed: "ssaaedd",
        },
        Input {
            name: "vtkgn",
            typed: "vttkgnn",
        },
    ];

    for input in inputs {
        let result =
            Solution::is_long_pressed_name(input.name.to_string(), input.typed.to_string());
        println!("{result:?}");
    }
}
