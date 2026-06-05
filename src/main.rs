struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1 = Self::split(num1);
        let num2 = Self::split(num2);

        let real = num1.0 * num2.0 - num1.1 * num2.1;
        let imagine = num1.0 * num2.1 + num2.0 * num1.1;
        format!("{}+{}i", real, imagine)
    }

    fn split(num: String) -> (i32, i32) {
        let mut splitted = num.split('+');
        let real = splitted.next().unwrap();
        let imagine = splitted.next().unwrap();
        let imagine = &imagine[0..(imagine.len() - 1)];

        (real.parse().unwrap(), imagine.parse().unwrap())
    }
}

struct Input {
    num1: String,
    num2: String,
}

fn main() {
    let inputs = [Input {
        num1: "1+1i".to_string(),
        num2: "1+1i".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::complex_number_multiply(input.num1, input.num2);
        println!("{:?}", result);
    }
}
