struct Solution {}

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let k = (k as u64) - 1;
        let result = (0..64 - k.leading_zeros())
            .rev()
            .fold(0i32, |acc, i| match (k >> i) & 1 {
                1 => acc + operations[i as usize],
                _ => acc,
            });

        (b'a' + (result % 26) as u8) as char
    }
}

struct Input {
    k: i64,
    operations: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            k: 5,
            operations: vec![0, 0, 0],
        },
        Input {
            k: 10,
            operations: vec![0, 1, 0, 1],
        },
    ];

    for input in inputs {
        let result = Solution::kth_character(input.k, input.operations);
        println!("{:?}", result);
    }
}
