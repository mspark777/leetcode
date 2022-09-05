struct Solution {}
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::<u8>::new();

        for ch in s.as_bytes() {
            if *ch != b']' {
                stack.push(*ch);
                continue;
            }

            let mut chars = Vec::<u8>::new();
            while let Some(top) = stack.pop() {
                if top != b'[' {
                    chars.push(top);
                } else {
                    break;
                }
            }

            let mut nums = Vec::<u8>::new();
            while let Some(top) = stack.last() {
                let top = *top;
                if (b'0' <= top) && (top <= b'9') {
                    nums.push(top);
                    stack.pop();
                } else {
                    break;
                }
            }

            chars.reverse();
            nums.reverse();
            let count = String::from_utf8(nums)
                .expect("no")
                .parse::<usize>()
                .expect("no");

            for _ in 0..count {
                let mut temp = chars.clone();
                stack.append(&mut temp);
            }
        }

        String::from_utf8(stack).expect("no")
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { s: "3[a]2[bc]" },
        Input { s: "3[a2[c]]" },
        Input { s: "2[abc]3[cd]ef" },
    ];

    for input in inputs.iter() {
        let s = input.s.to_string();
        let result = Solution::decode_string(s);
        println!("{result:?}");
    }
}
