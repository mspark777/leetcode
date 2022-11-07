struct Solution {}
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        return num.to_string().replacen("6", "9", 1).parse().unwrap();
    }
}

fn main() {
    let inputs = [9669, 9996, 9999];

    for num in inputs {
        let result = Solution::maximum69_number(num);
        println!("{result}");
    }
}
