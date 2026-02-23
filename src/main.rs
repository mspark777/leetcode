struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let origins = ['a', '0'].map(|o| o as u32);
        let p1 = coordinate1
            .chars()
            .zip(origins)
            .map(|(c, o)| (c as u32) - o)
            .sum::<u32>();
        let p2 = coordinate2
            .chars()
            .zip(origins)
            .map(|(c, o)| (c as u32) - o)
            .sum::<u32>();

        (p1 & 1) == (p2 & 1)
    }
}

struct Input {
    coordinate1: String,
    coordinate2: String,
}

fn main() {
    let inputs = [
        Input {
            coordinate1: "a1".to_string(),
            coordinate2: "c3".to_string(),
        },
        Input {
            coordinate1: "a1".to_string(),
            coordinate2: "h3".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::check_two_chessboards(input.coordinate1, input.coordinate2);
        println!("{:?}", result);
    }
}
