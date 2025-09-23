struct Solution {}

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        word1
            .chars()
            .zip(word2.chars())
            .fold(vec![0; 26], |mut counts, (left, right)| {
                let a = 'a' as usize;
                let lcode = left as usize - a;
                let rcode = right as usize - a;
                counts[lcode] += 1;
                counts[rcode] -= 1;
                counts
            })
            .iter()
            .all(|&c| c >= -3 && c <= 3)
    }
}

struct Input {
    word1: String,
    word2: String,
}

fn main() {
    let inputs = [
        Input {
            word1: "aaaa".to_string(),
            word2: "bccb".to_string(),
        },
        Input {
            word1: "abcdeef".to_string(),
            word2: "abaaacc".to_string(),
        },
        Input {
            word1: "cccddabba".to_string(),
            word2: "babababab".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::check_almost_equivalent(input.word1, input.word2);
        println!("{:?}", result);
    }
}
