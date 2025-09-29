struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut result = title.chars().collect::<Vec<char>>();
        let mut last = 0usize;
        let n = result.len();

        while last < n {
            let start = last;
            while (last < n) && (result[last] != ' ') {
                result[last] = result[last].to_ascii_lowercase();
                last += 1;
            }

            if (last - start) > 2 {
                result[start] = result[start].to_ascii_uppercase();
            }

            last += 1;
        }

        result.iter().collect()
    }
}

struct Input {
    title: String,
}

fn main() {
    let inputs = [
        Input {
            title: "capiTalIze tHe titLe".to_string(),
        },
        Input {
            title: "First leTTeR of EACH Word".to_string(),
        },
        Input {
            title: "i lOve leetcode".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::capitalize_title(input.title);
        println!("{:?}", result);
    }
}
