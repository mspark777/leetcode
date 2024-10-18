struct Solution {}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for n in left..=right {
            if Self::self_dividing(n) {
                result.push(n);
            }
        }

        return result;
    }

    fn self_dividing(n: i32) -> bool {
        let mut i = n;

        while i > 0 {
            let d = i % 10;
            if d == 0 {
                return false;
            } else if (n % d) > 0 {
                return false;
            }

            i /= 10;
        }

        return true;
    }
}

fn main() {
    let inputs = vec![(1, 22), (47, 85)];

    for input in inputs {
        let result = Solution::self_dividing_numbers(input.0, input.1);
        println!("{result:?}");
    }
}
