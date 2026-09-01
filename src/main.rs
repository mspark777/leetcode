struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::vec_deque::VecDeque;

        let mut deq1: VecDeque<_> = arr1.into();
        let mut deq2: VecDeque<_> = arr2.into();
        let mut result: VecDeque<i32> = VecDeque::new();
        let mut carry: i32 = 0;
        while !deq1.is_empty() || !deq2.is_empty() {
            let i1: i32 = deq1.pop_back().unwrap_or(0);
            let i2: i32 = deq2.pop_back().unwrap_or(0);
            let i = i1 + i2 + (carry & 0b1);
            result.push_front(i & 0b1);
            carry = Self::new_carry(
                carry,
                match i > 1 {
                    true => 0b11,
                    _ => 0,
                },
            );
        }
        result.push_front(carry & 0b1);
        result.push_front((carry >> 1) & 0b1);

        while result.front() == Some(&0) && result.len() > 1 {
            result.pop_front();
        }

        result.into()
    }

    fn new_carry(orig: i32, next: i32) -> i32 {
        ((orig >> 1) + next) & 0b11
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
