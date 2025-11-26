struct Solution {}

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n).filter(Self::check).sum()
    }

    fn check(n: &i32) -> bool {
        let n = *n;
        let mod3 = n % 3;
        let mod5 = n % 5;
        let mod7 = n % 7;

        (mod3 == 0) || (mod5 == 0) || (mod7 == 0)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 7 }, Input { n: 10 }, Input { n: 9 }];

    for input in inputs {
        let result = Solution::sum_of_multiples(input.n);
        println!("{:?}", result);
    }
}
