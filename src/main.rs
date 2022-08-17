struct Solution {}
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut num = x;
        num = ((num & 0xFFFF0000) >> 16) | ((num & 0x0000FFFF) << 16);
        num = ((num & 0xFF00FF00) >> 8) | ((num & 0x00FF00FF) << 8);
        num = ((num & 0xF0F0F0F0) >> 4) | ((num & 0x0F0F0F0F) << 4);
        num = ((num & 0xCCCCCCCC) >> 2) | ((num & 0x33333333) << 2);
        num = ((num & 0xAAAAAAAA) >> 1) | ((num & 0x55555555) << 1);

        num
    }
}

struct Input {
    x: u32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { x: 43261596 }, Input { x: 4294967293 }];

    for input in inputs {
        let x = input.x;
        let result = Solution::reverse_bits(x);
        println!("{:?}", result);
    }
}
