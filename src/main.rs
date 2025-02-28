struct Solution {}

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let str1 = str1.chars().collect::<Vec<char>>();
        let str2 = str2.chars().collect::<Vec<char>>();

        let str1_len = str1.len();
        let str2_len = str2.len();
        let mut dp = vec![vec![0; str2_len + 1]; str1_len + 1];

        for row in 0..=str1_len {
            dp[row][0] = row;
        }

        for col in 0..=str2_len {
            dp[0][col] = col;
        }

        for row in 1..=str1_len {
            for col in 1..=str2_len {
                if str1[row - 1] == str2[col - 1] {
                    dp[row][col] = dp[row - 1][col - 1] + 1;
                } else {
                    dp[row][col] = dp[row - 1][col].min(dp[row][col - 1]) + 1;
                }
            }
        }

        let mut supersequence = Vec::<char>::new();
        let mut row = str1_len;
        let mut col = str2_len;

        while row > 0 && col > 0 {
            if str1[row - 1] == str2[col - 1] {
                supersequence.push(str1[row - 1]);
                row -= 1;
                col -= 1;
            } else if dp[row - 1][col] < dp[row][col - 1] {
                supersequence.push(str1[row - 1]);
                row -= 1;
            } else {
                supersequence.push(str2[col - 1]);
                col -= 1;
            }
        }

        while row > 0 {
            supersequence.push(str1[row - 1]);
            row -= 1;
        }

        while col > 0 {
            supersequence.push(str2[col - 1]);
            col -= 1;
        }

        return supersequence.iter().rev().collect();
    }
}

struct Input {
    str1: &'static str,
    str2: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            str1: "abac",
            str2: "cab",
        },
        Input {
            str1: "aaaaaaaa",
            str2: "aaaaaaaa",
        },
    ];

    for input in inputs {
        let result =
            Solution::shortest_common_supersequence(input.str1.to_string(), input.str2.to_string());
        println!("{result:?}");
    }
}
