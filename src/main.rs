struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let s = s.as_bytes();
        let mut last_pos = [usize::MAX; 26];
        for i in 0..s.len() {
            last_pos[s[i] as usize - 97] = i;
        }

        let mut stack = Vec::with_capacity(26);
        let mut seen = [false; 26];
        for i in 0..s.len() {
            let c = s[i] as usize - 97;
            if seen[c] {
                continue;
            }

            while !stack.is_empty()
                && stack[stack.len() - 1] > c
                && last_pos[stack[stack.len() - 1]] > i
            {
                seen[stack[stack.len() - 1]] = false;
                stack.pop();
            }
            seen[c] = true;
            stack.push(c);
        }

        let stack = stack.iter().map(|x| *x as u8 + 97).collect::<Vec<u8>>();
        return String::from_utf8(stack).unwrap();
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
