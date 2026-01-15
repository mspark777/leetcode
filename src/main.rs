struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut n = word.len() as i32;
        let mut result = 0;
        let mut count = 1;

        while n >= 8 {
            n -= 8;
            result += count * 8;
            count += 1;
        }

        result += count * n;
        result
    }
}

struct Input {
    word: String,
}

fn main() {
    let inputs = [
        Input {
            word: "abcde".to_string(),
        },
        Input {
            word: "xycdefghij".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_pushes(input.word);
        println!("{:?}", result);
    }
}
