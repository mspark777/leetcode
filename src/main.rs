struct Solution {}

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize;
        let mut result = 0;
        let mut alternating_elements_count = 1;
        let mut last_color = colors[0];

        for i in 1..(n + k - 1) {
            let idx = i % n;

            if colors[idx] == last_color {
                alternating_elements_count = 1;
                last_color = colors[idx];
                continue;
            }

            alternating_elements_count += 1;

            if alternating_elements_count >= k {
                result += 1;
            }

            last_color = colors[idx];
        }

        return result;
    }
}

struct Input {
    colors: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            colors: vec![0, 1, 0, 1, 0],
            k: 3,
        },
        Input {
            colors: vec![0, 1, 0, 0, 1, 0, 1],
            k: 6,
        },
        Input {
            colors: vec![1, 1, 0, 1],
            k: 4,
        },
    ];

    for input in inputs {
        let result = Solution::number_of_alternating_groups(input.colors, input.k);
        println!("{result:?}");
    }
}
