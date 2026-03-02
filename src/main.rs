struct Solution;

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut result = events[0][0];
        let mut prev_time = events[0][1];
        let mut max_time = prev_time;

        for event in events.iter().skip(1) {
            let index = event[0];
            let time = event[1];
            let taken = time - prev_time;

            if max_time < taken {
                result = index;
                max_time = taken;
            } else if max_time == taken {
                result = result.min(index);
            }

            prev_time = time;
        }

        result
    }
}

struct Input {
    events: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            events: [[1, 2], [2, 5], [3, 9], [1, 15]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            events: [[10, 5], [1, 7]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::button_with_longest_time(input.events);
        println!("{:?}", result);
    }
}
