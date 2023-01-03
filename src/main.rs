struct Solution {}
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut result = 0;

        for c in 0..strs[0].len() {
            for r in 1..strs.len() {
                let c0 = strs[r - 1].as_bytes()[c];
                let c1 = strs[r].as_bytes()[c];
                if c0 > c1 {
                    result += 1;
                    break;
                }
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec!["cba", "daf", "ghi"],
        vec!["a", "b"],
        vec!["zyx", "wvu", "tsr"],
    ];

    for strs in inputs {
        let result = Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect());
        println!("{result}");
    }
}
