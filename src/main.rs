struct Solution {}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let alice_arrive = Self::to_offset(arrive_alice.as_str());
        let alice_leave = Self::to_offset(leave_alice.as_str());
        let bob_arrive = Self::to_offset(arrive_bob.as_str());
        let bob_leave = Self::to_offset(leave_bob.as_str());

        let arrive = alice_arrive.max(bob_arrive);
        let leave = alice_leave.min(bob_leave);
        match arrive <= leave {
            true => leave - arrive + 1,
            _ => 0,
        }
    }

    fn parse(s: &str) -> (usize, i32) {
        let mut split = s.split('-');
        let m = split.next().unwrap();
        let d = split.next().unwrap();

        let month = m.parse::<usize>().unwrap() - 1;
        let day = d.parse::<i32>().unwrap();
        (month, day)
    }

    fn to_offset(s: &str) -> i32 {
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let (month, day) = Self::parse(s);
        let mut offset = day;
        for d in days.iter().take(month) {
            offset += d
        }

        offset
    }
}

struct Input {
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
}

fn main() {
    let inputs = [
        Input {
            arrive_alice: "08-15".to_string(),
            leave_alice: "08-18".to_string(),
            arrive_bob: "08-16".to_string(),
            leave_bob: "08-19".to_string(),
        },
        Input {
            arrive_alice: "10-01".to_string(),
            leave_alice: "10-31".to_string(),
            arrive_bob: "11-01".to_string(),
            leave_bob: "12-31".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_days_together(
            input.arrive_alice,
            input.leave_alice,
            input.arrive_bob,
            input.leave_bob,
        );
        println!("{:?}", result);
    }
}
