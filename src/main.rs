struct Solution {}

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let first = Self::to_int(first_word.as_str());
        let second = Self::to_int(second_word.as_str());
        let target = Self::to_int(target_word.as_str());

        (first + second) == target
    }

    fn to_int(s: &str) -> i32 {
        s.chars().fold(0, |n, ch| {
            let cn = ch as i32;
            let a = 'a' as i32;
            (n * 10) + (cn - a)
        })
    }
}

struct Input {
    first_word: String,
    second_word: String,
    target_word: String,
}

fn main() {
    let inputs = [
        Input {
            first_word: "acb".to_string(),
            second_word: "cba".to_string(),
            target_word: "cdb".to_string(),
        },
        Input {
            first_word: "aaa".to_string(),
            second_word: "a".to_string(),
            target_word: "aab".to_string(),
        },
        Input {
            first_word: "aaa".to_string(),
            second_word: "a".to_string(),
            target_word: "aaaa".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_sum_equal(input.first_word, input.second_word, input.target_word);
        println!("{:?}", result);
    }
}
