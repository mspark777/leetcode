struct Solution {}

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = Vec::<char>::with_capacity(chars.len() + spaces.len());

        let mut space_index = 0usize;
        let mut string_index = 0usize;

        while string_index < chars.len() {
            while space_index < spaces.len() {
                let space = spaces[space_index] as usize;
                if string_index == space {
                    result.push(' ');
                    space_index += 1;
                } else {
                    break;
                }
            }

            result.push(chars[string_index]);
            string_index += 1;
        }

        return result.iter().collect();
    }
}

struct Input {
    s: &'static str,
    spaces: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            s: "LeetcodeHelpsMeLearn",
            spaces: vec![8, 13, 15],
        },
        Input {
            s: "icodeinpython",
            spaces: vec![1, 5, 7, 9],
        },
        Input {
            s: "spacing",
            spaces: vec![0, 1, 2, 3, 4, 5, 6],
        },
    ];

    for input in inputs {
        let result = Solution::add_spaces(input.s.to_string(), input.spaces);
        println!("{result}");
    }
}
