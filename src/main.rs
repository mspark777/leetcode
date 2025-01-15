struct Solution {}

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        return Self::solve(num1 as u32, num2 as u32) as i32;
    }

    fn solve(num1: u32, num2: u32) -> u32 {
        let mut result = 0u32;
        let target_set_bits_count = num2.count_ones();
        let mut set_bits_count = 0u32;
        let mut current_bit = 31u32;

        while set_bits_count < target_set_bits_count {
            if Self::is_set(num1, current_bit) {
                result = Self::set_bit(result, current_bit);
                set_bits_count += 1;
                current_bit = Self::next_current_bit(current_bit);
                continue;
            }

            let bit_diff = target_set_bits_count - set_bits_count;
            if bit_diff > current_bit {
                result = Self::set_bit(result, current_bit);
                set_bits_count += 1;
            }
            current_bit = Self::next_current_bit(current_bit);
        }

        return result;
    }

    fn is_set(x: u32, bit: u32) -> bool {
        return (x & (1 << bit)) != 0;
    }

    fn set_bit(x: u32, bit: u32) -> u32 {
        return x | (1 << bit);
    }

    fn next_current_bit(current: u32) -> u32 {
        return if current > 1 { current - 1 } else { 0 };
    }
}

struct Input {
    num1: i32,
    num2: i32,
}

fn main() {
    let inputs = vec![Input { num1: 3, num2: 5 }, Input { num1: 1, num2: 12 }];

    for input in inputs {
        let result = Solution::minimize_xor(input.num1, input.num2);
        println!("{result:?}");
    }
}
