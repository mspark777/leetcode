struct Solution {}

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let start = event1[0].as_str().max(event2[0].as_str());
        let end = event1[0].as_str().min(event2[1].as_str());

        match start.cmp(end) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
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
