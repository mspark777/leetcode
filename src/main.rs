struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut first_diff = (' ', ' ');
        let mut second_diff = (' ', ' ');
        let mut num_diffs = 0usize;

        for (ch1, ch2) in s1.chars().zip(s2.chars()) {
            if ch1 == ch2 {
                continue;
            }

            num_diffs += 1;
            if num_diffs > 2 {
                return false;
            } else if num_diffs == 1 {
                first_diff.0 = ch1;
                first_diff.1 = ch2;
            } else {
                second_diff.0 = ch1;
                second_diff.1 = ch2;
            }
        }

        return (first_diff.0 == second_diff.1) && (first_diff.1 == second_diff.0);
    }
}

struct Input {
    s1: &'static str,
    s2: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            s1: "bank",
            s2: "kanb",
        },
        Input {
            s1: "attack",
            s2: "defend",
        },
        Input {
            s1: "kelb",
            s2: "kelb",
        },
    ];

    for input in inputs {
        let result = Solution::are_almost_equal(input.s1.to_string(), input.s2.to_string());
        println!("{result:?}");
    }
}
