struct Solution {}

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        return Self::solve(start.chars().collect(), target.chars().collect());
    }

    fn solve(start: Vec<char>, target: Vec<char>) -> bool {
        let char_count = start.len();
        let mut start_index = 0usize;
        let mut target_index = 0usize;

        while start_index < char_count || target_index < char_count {
            while start_index < char_count && start[start_index] == '_' {
                start_index += 1;
            }

            while target_index < char_count && target[target_index] == '_' {
                target_index += 1;
            }

            if start_index == char_count || target_index == char_count {
                return start_index == char_count && target_index == char_count;
            }

            if start[start_index] != target[target_index]
                || (start[start_index] == 'L' && start_index < target_index)
                || (start[start_index] == 'R' && start_index > target_index)
            {
                return false;
            }

            start_index += 1;
            target_index += 1;
        }

        return true;
    }
}

struct Input {
    start: &'static str,
    target: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            start: "_L__R__R_",
            target: "L______RR",
        },
        Input {
            start: "R_L_",
            target: "__LR",
        },
        Input {
            start: "_R",
            target: "R_",
        },
    ];

    for input in inputs {
        let result = Solution::can_change(input.start.to_string(), input.target.to_string());
        println!("{result}");
    }
}
