struct Solution {}

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        s.chars()
            .enumerate()
            .fold(
                (0i32, 0i32, 0i32),
                |(latitude, longitude, result), (i, c)| {
                    let latitude = match c {
                        'N' => latitude + 1,
                        'S' => latitude - 1,
                        _ => latitude,
                    };

                    let longitude = match c {
                        'E' => longitude + 1,
                        'W' => longitude - 1,
                        _ => longitude,
                    };

                    let j = (i + 1) as i32;
                    let current = j.min(latitude.abs() + longitude.abs() + k * 2);
                    (latitude, longitude, result.max(current))
                },
            )
            .2
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![Input { s: "NWSE", k: 1 }, Input { s: "NSWWEW", k: 3 }];

    for input in inputs {
        let result = Solution::max_distance(input.s.to_string(), input.k);
        println!("{:?}", result);
    }
}
