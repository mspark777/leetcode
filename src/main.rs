struct Solution {}

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let event1_start = Self::parse(event1[0].as_str());
        let event1_end = Self::parse(event1[1].as_str());

        let event2_start = Self::parse(event2[0].as_str());
        let event2_end = Self::parse(event2[1].as_str());

        let start = event1_start.max(event2_start);
        let end = event1_end.min(event2_end);

        start <= end
    }

    fn parse(s: &str) -> i32 {
        let scales = [60, 1];
        s.split(':')
            .zip(scales.iter().cloned())
            .fold(0, |acc, (chunk, scale)| {
                acc + chunk.parse::<i32>().unwrap() * scale
            })
    }
}

struct Input {
    event1: Vec<String>,
    event2: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            event1: ["01:15", "02:00"].map(|s| s.to_string()).to_vec(),
            event2: ["02:00", "03:00"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            event1: ["01:00", "02:00"].map(|s| s.to_string()).to_vec(),
            event2: ["01:20", "03:00"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            event1: ["10:00", "11:00"].map(|s| s.to_string()).to_vec(),
            event2: ["14:00", "15:00"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            event1: ["05:10", "15:05"].map(|s| s.to_string()).to_vec(),
            event2: ["14:59", "19:17"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::have_conflict(input.event1, input.event2);
        println!("{:?}", result);
    }
}
