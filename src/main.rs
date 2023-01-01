struct Solution {}
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0usize;
        let mut j = s.len() - 1;
        while i < j {
            let si = s[i];
            let sj = s[j];
            s[i] = sj;
            s[j] = si;
            i += 1;
            j -= 1;
        }
    }
}

fn main() {
    let mut inputs = [
        vec!['h', 'e', 'l', 'l', 'o'],
        vec!['H', 'a', 'n', 'n', 'a', 'h'],
    ];

    for s in inputs.iter_mut() {
        Solution::reverse_string(s);
        println!("{s:?}");
    }
}
