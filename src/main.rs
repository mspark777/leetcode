struct Solution {}
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut i = n;
        if i <= 0 {
            return false;
        }

        while (i % 3) == 0 {
            i /= 3;
        }

        i == 1
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { n: 27 },
        Input { n: 0 },
        Input { n: 9 },
        Input { n: 45 },
    ];

    for input in inputs.iter() {
        let n = input.n;
        let result = Solution::is_power_of_three(n);
        println!("{:?}", result);
    }
}
