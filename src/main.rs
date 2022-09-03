struct StackNode {
    len: i32,
    num: i32,
    digit: i32,
}

impl StackNode {
    #[inline]
    fn new(len: i32, num: i32, digit: i32) -> Self {
        StackNode { len, num, digit }
    }
}
struct Solution {}
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut stack = Vec::<StackNode>::new();
        let mut result = Vec::<i32>::new();

        for i in 1..10 {
            stack.push(StackNode::new(n - 1, i, i));
        }

        while let Some(top) = stack.pop() {
            let StackNode { len, num, digit } = top;
            if len == 0 {
                result.push(num);
                continue;
            }

            for i in 0..10 {
                if (i - digit).abs() == k {
                    stack.push(StackNode::new(len - 1, num * 10 + i, i));
                }
            }
        }

        result
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 3, k: 7 }, Input { n: 2, k: 1 }];

    for input in inputs.iter() {
        let result = Solution::nums_same_consec_diff(input.n, input.k);
        println!("{result:?}");
    }
}
