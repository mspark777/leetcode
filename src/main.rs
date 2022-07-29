mod solution;

use solution::Solution;

struct Input {
    words: Vec<String>,
    pattern: String,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            words: vec![
                String::from("abc"),
                String::from("deq"),
                String::from("mee"),
                String::from("aqq"),
                String::from("dkd"),
                String::from("ccc"),
            ],
            pattern: "abb".to_owned(),
        },
        Input {
            words: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            pattern: "a".to_owned(),
        },
    ];

    for input in inputs {
        let result = Solution::find_and_replace_pattern(input.words, input.pattern);
        println!("{:?}", result);
    }
}
