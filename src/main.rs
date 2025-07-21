struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut b_count = 0;
        let mut a_count = 0;
        let mut l_count = 0;
        let mut o_count = 0;
        let mut n_count = 0;

        for ch in text.chars() {
            match ch {
                'b' => b_count += 1,
                'a' => a_count += 1,
                'l' => l_count += 1,
                'o' => o_count += 1,
                'n' => n_count += 1,
                _ => (),
            };
        }

        l_count /= 2;
        o_count /= 2;
        b_count.min(a_count).min(l_count).min(o_count).min(n_count)
    }
}

struct Input {
    text: String,
}

fn main() {
    let inputs = vec![
        Input {
            text: "nlaebolko".to_string(),
        },
        Input {
            text: "loonbalxballpoon".to_string(),
        },
        Input {
            text: "leetcode".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::max_number_of_balloons(input.text);
        println!("{:?}", result);
    }
}
