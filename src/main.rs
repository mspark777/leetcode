struct Solution {}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut meeting_count = vec![0; n];
        let mut used_rooms =
            std::collections::BinaryHeap::<std::cmp::Reverse<(usize, usize)>>::new();
        let mut unused_rooms = std::collections::BinaryHeap::<std::cmp::Reverse<usize>>::from_iter(
            (0..n).map(|v| std::cmp::Reverse(v)),
        );

        let mut meetings = meetings;
        meetings.sort_unstable_by(|l, r| {
            if l[0] == r[0] {
                l[1].cmp(&r[1])
            } else {
                l[0].cmp(&r[0])
            }
        });

        for meeting in meetings.iter() {
            let start = meeting[0] as usize;
            let end = meeting[1] as usize;

            while let Some(std::cmp::Reverse(top)) = used_rooms.peek() {
                if top.0 <= start {
                    let room = top.1;
                    used_rooms.pop();
                    unused_rooms.push(std::cmp::Reverse(room));
                } else {
                    break;
                }
            }

            if let Some(std::cmp::Reverse(room)) = unused_rooms.pop() {
                used_rooms.push(std::cmp::Reverse((end, room)));
                meeting_count[room] += 1;
            } else if let Some(std::cmp::Reverse((availability_time, room))) = used_rooms.pop() {
                used_rooms.push(std::cmp::Reverse((availability_time + end - start, room)));
                meeting_count[room] += 1;
            }
        }

        let mut max_meeting_count = 0;
        let mut result = 0;
        for (i, count) in meeting_count.iter().cloned().enumerate() {
            if count > max_meeting_count {
                max_meeting_count = count;
                result = i as i32;
            }
        }

        return result;
    }
}

struct Input {
    n: i32,
    meetings: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 2,
            meetings: [[0, 10], [1, 5], [2, 7], [3, 4]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
        },
        Input {
            n: 3,
            meetings: [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
        },
    ];

    for input in inputs {
        let result = Solution::most_booked(input.n, input.meetings);
        println!("{:?}", result);
    }
}
