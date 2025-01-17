struct Solution {}

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut xor = 0u32;
        for n in derived.iter().cloned() {
            xor ^= n as u32;
        }

        return xor == 0;
    }
}

struct Input {
    derived: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            derived: vec![1, 1, 0],
        },
        Input {
            derived: vec![1, 1],
        },
        Input {
            derived: vec![1, 0],
        },
    ];

    for input in inputs {
        let result = Solution::does_valid_array_exist(input.derived);
        println!("{result:?}");
    }
}
