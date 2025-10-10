struct Solution {}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let curr = Self::to_int(current.as_str());
        let correct = Self::to_int(correct.as_str());
        let mut count = 0;

        let mut diff = correct - curr;
        for by in [60, 15, 5] {
            let temp = diff / by;
            count += temp;
            diff -= temp * by;
        }

        count + diff
    }

    fn to_int(s: &str) -> i32 {
        let mut chunks = s.split(':');
        let left = match chunks.next() {
            Some(v) => v.parse::<i32>().unwrap(),
            _ => 0,
        };

        let right = match chunks.next() {
            Some(v) => v.parse::<i32>().unwrap(),
            _ => 0,
        };

        left * 60 + right
    }
}

struct Input {
    current: String,
    correct: String,
}

fn main() {
    let inputs = [
        Input {
            current: "02:30".to_string(),
            correct: "04:35".to_string(),
        },
        Input {
            current: "11:00".to_string(),
            correct: "11:01".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::convert_time(input.current, input.correct);
        println!("{:?}", result);
    }
}
