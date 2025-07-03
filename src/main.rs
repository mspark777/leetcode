struct Solution {}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut result = b'a';
        let mut k = k;

        while k != 1 {
            let mut t = 31 - k.leading_zeros();
            if (1 << t) == k {
                t -= 1
            }

            k -= 1 << t;
            result += 1;
        }

        result as char
    }
}

struct Input {
    k: i32,
}

fn main() {
    let inputs = vec![Input { k: 5 }, Input { k: 10 }];

    for input in inputs {
        let result = Solution::kth_character(input.k);
        println!("{:?}", result);
    }
}
