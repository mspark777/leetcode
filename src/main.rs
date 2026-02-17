struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut result = 0;
        for (i, current) in colors.iter().copied().enumerate() {
            let i1 = (i + 1) % n;
            let i2 = (i + 2) % n;
            let next1 = colors[i1];
            let next2 = colors[i2];

            match (current, next1, next2) {
                (1, 0, 1) | (0, 1, 0) => result += 1,
                _ => continue,
            };
        }

        result
    }
}

struct Input {
    colors: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            colors: [1, 1, 1].to_vec(),
        },
        Input {
            colors: [0, 1, 0, 0, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::number_of_alternating_groups(input.colors);
        println!("{:?}", result);
    }
}
