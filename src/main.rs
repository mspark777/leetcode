struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut result = 0;
        for i in left..=right {
            result += if Self::is_prime(i.count_ones()) { 1 } else { 0 };
        }

        return result;
    }

    fn is_prime(i: u32) -> bool {
        return if i == 2 {
            true
        } else if i == 3 {
            true
        } else if i == 5 {
            true
        } else if i == 7 {
            true
        } else if i == 11 {
            true
        } else if i == 13 {
            true
        } else if i == 17 {
            true
        } else if i == 19 {
            true
        } else {
            false
        };
    }
}

struct Input {
    left: i32,
    right: i32,
}

fn main() {
    let inputs = vec![
        Input { left: 6, right: 10 },
        Input {
            left: 10,
            right: 15,
        },
    ];

    for input in inputs {
        let result = Solution::count_prime_set_bits(input.left, input.right);
        println!("{result}");
    }
}
