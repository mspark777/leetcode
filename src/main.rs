mod solution;

struct Input {
    n: &'static str,
}

fn main() {
    let inputs = [
        Input { n: "32" },
        Input { n: "82734" },
        Input {
            n: "27346209830709182346",
        },
        Input { n: "135" },
    ];

    for input in inputs {
        let result = solution::Solution::min_partitions(String::from(input.n));
        println!("{result:?}");
    }
}
