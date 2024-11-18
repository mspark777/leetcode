struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let code_len = code.len();
        if k == 0 {
            return vec![0; code_len];
        }

        let key_len = k.abs() as usize;
        let mut start = 1usize;
        let mut end = key_len;

        if k < 0 {
            start = code_len - key_len;
            end = code_len - 1;
        }

        let mut result = vec![0; code_len];
        let mut sum = 0;
        for i in start..=end {
            sum += code[i];
        }

        for i in 0..code_len {
            result[i] = sum;

            sum -= code[start % code_len];

            start += 1;
            end += 1;

            sum += code[end % code_len];
        }

        return result;
    }
}

struct Input {
    code: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            code: vec![5, 7, 1, 4],
            k: 3,
        },
        Input {
            code: vec![1, 2, 3, 4],
            k: 0,
        },
        Input {
            code: vec![2, 4, 9, 3],
            k: -2,
        },
    ];

    for input in inputs {
        let result = Solution::decrypt(input.code, input.k);
        println!("{result:?}");
    }
}
