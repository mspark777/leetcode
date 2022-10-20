const M: [&'static str; 4] = ["", "M", "MM", "MMM"];
const C: [&'static str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const X: [&'static str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const I: [&'static str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mi = num / 1000;
        let ci = (num % 1000) / 100;
        let xi = (num % 100) / 10;
        let ii = num % 10;

        let m = M[mi as usize];
        let c = C[ci as usize];
        let x = X[xi as usize];
        let i = I[ii as usize];

        return format!("{m}{c}{x}{i}");
    }
}

fn main() {
    let inputs = [3, 58, 1994];

    for num in inputs {
        let result = Solution::int_to_roman(num);
        println!("{result}");
    }
}
