struct Solution {}
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let counts = Self::get_counts(n);

        for i in 0..32 {
            if counts == Self::get_counts(1 << i) {
                return true;
            }
        }

        false
    }

    fn get_counts(n: i32) -> Vec<i32> {
        let mut n = n;
        let mut result = vec![0; 10];
        while n > 0 {
            let idx = (n % 10) as usize;
            result[idx] += 1;
            n /= 10;
        }

        return result;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 1 }, Input { n: 10 }, Input { n: 46 }];

    for input in inputs.iter() {
        let n = input.n;
        let result = Solution::reordered_power_of2(n);
        println!("{result}");
    }
}
