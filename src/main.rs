struct Solution {}
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let fb = buckets as f64;
        let fd = minutes_to_die as f64;
        let ft = minutes_to_test as f64;

        (fb.ln() / (ft / fd + 1.0).ln()).ceil() as i32
    }
}

struct Input {
    buckets: i32,
    minutes_to_die: i32,
    minutes_to_test: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            buckets: 1000,
            minutes_to_die: 15,
            minutes_to_test: 60,
        },
        Input {
            buckets: 4,
            minutes_to_die: 15,
            minutes_to_test: 15,
        },
        Input {
            buckets: 4,
            minutes_to_die: 15,
            minutes_to_test: 30,
        },
    ];

    for input in inputs {
        let buckets = input.buckets;
        let minutes_to_die = input.minutes_to_die;
        let minutes_to_test = input.minutes_to_test;
        let result = Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test);
        println!("{:?}", result);
    }
}
