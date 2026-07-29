struct Solution;

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut in_block = false;
        let mut new_line = String::new();
        let mut result = Vec::<String>::new();

        for line in source {
            let mut i = 0usize;
            let line = line.as_str();
            let n = line.len();
            let last = n - 1;

            while i < n {
                if in_block {
                    if i < last && &line[i..(i + 2)] == "*/" {
                        in_block = false;
                        i += 1;
                    }
                } else {
                    if i < last && &line[i..(i + 2)] == "//" {
                        break;
                    } else if i < last && &line[i..(i + 2)] == "/*" {
                        in_block = true;
                        i += 1;
                    } else {
                        new_line.push_str(&line[i..(i + 1)]);
                    }
                }
                i += 1;
            }

            if !in_block && !new_line.is_empty() {
                result.push(new_line);
                new_line = String::new();
            }
        }

        result
    }
}

struct Input {
    source: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            source: [
                "/*Test program */",
                "int main()",
                "{ ",
                "  // variable declaration ",
                "int a, b, c;",
                "/* This is a test",
                "   multiline  ",
                "   comment for ",
                "   testing */",
                "a = b + c;",
                "}",
            ]
            .map(|s| s.to_string())
            .to_vec(),
        },
        Input {
            source: ["a/*comment", "line", "more_comment*/b"]
                .map(|s| s.to_string())
                .to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::remove_comments(input.source);
        println!("{:?}", result);
    }
}
