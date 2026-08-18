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
