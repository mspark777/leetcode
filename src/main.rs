struct Solution {}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}

struct Input {
    celsius: f64,
}

fn main() {
    let inputs = [Input { celsius: 36.50 }, Input { celsius: 122.11 }];

    for input in inputs {
        let result = Solution::convert_temperature(input.celsius);
        println!("{:?}", result);
    }
}
