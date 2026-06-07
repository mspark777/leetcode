struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() > 1440 {
            return 0;
        }

        let mut minutes = time_points
            .iter()
            .map(|s| Self::parse(s.as_str()))
            .collect::<Vec<i32>>();

        minutes.sort_unstable();
        minutes.push(minutes[0] + 1440);
        minutes.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }

    fn parse(s: &str) -> i32 {
        let h = s[..2].parse::<i32>().unwrap();
        let m = s[3..].parse::<i32>().unwrap();

        h * 60 + m
    }
}

struct Input {
    time_points: Vec<String>,
}

fn main() {
    let inputs = [Input {
        time_points: ["23:59", "00:00"].map(|s| s.to_string()).to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_min_difference(input.time_points);
        println!("{:?}", result);
    }
}
