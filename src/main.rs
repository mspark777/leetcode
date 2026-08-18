use std::collections::BTreeSet;

struct ExamRoom {
    size: i32,
    seats: BTreeSet<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            size: n,
            seats: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.insert(0);
            return 0;
        }

        let mut len = 0;
        let mut result = 0;
        if let Some(k) = self.seats.iter().next().copied()
            && (k > 0)
        {
            len = k;
            result = 0;
        }

        let mut last = -1;
        for k in self.seats.iter().copied() {
            if last == -1 {
                last = k;
                continue;
            }

            if (k - last) / 2 > len {
                result = last + 1 + ((k - last - 2) / 2);
                len = (k - last) / 2;
            }
            last = k;
        }

        if let Some(k) = self.seats.iter().next_back().copied()
            && ((self.size - 1 - k) > len)
        {
            result = self.size - 1;
        }

        self.seats.insert(result);
        result
    }

    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}

struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut v = Vec::<(i32, i32)>::from_iter(position.into_iter().zip(speed));
        v.sort_by_key(|a| a.0);

        let mut result = 0;
        v.iter()
            .map(|&(p, s)| {
                let target = target as f32;
                let p = p as f32;
                let s = s as f32;
                (target - p) / s
            })
            .rev()
            .fold(f32::MIN, |cur, t| {
                if t > cur {
                    result += 1;
                    t
                } else {
                    cur
                }
            });

        result
    }
}

struct Input {
    target: i32,
    position: Vec<i32>,
    speed: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        target: 12,
        position: [10, 8, 0, 5, 3].to_vec(),
        speed: [2, 4, 1, 1, 3].to_vec(),
    }];

    for input in inputs {
        let result = Solution::car_fleet(input.target, input.position, input.speed);
        println!("{:?}", result);
    }
}
