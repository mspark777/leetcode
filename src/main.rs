struct Solution {}
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        return s
            .chars()
            .fold((0, 0), |(result, num), c| match c {
                '0' => (num.min(result + 1), num),
                _ => (result, num + 1),
            })
            .0;
    }
}

fn main() {
    let inputs = ["00110", "010110", "00011000"];

    for input in inputs {
        let result = Solution::min_flips_mono_incr(input.to_string());
        println!("{result}");
    }
}
