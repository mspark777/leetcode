struct Solution {}
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut bytes = 0;

        for i in data.into_iter() {
            if bytes == 0 {
                let mut mask = 128;
                while (mask & i) != 0 {
                    bytes += 1;
                    mask >>= 1;
                }

                if bytes == 0 {
                    continue;
                }

                if (bytes > 4) || (bytes == 1) {
                    return false;
                }
            } else {
                if ((i & 128) == 0) || ((i & 64) != 0) {
                    return false;
                }
            }

            bytes -= 1;
        }

        bytes == 0
    }
}

struct Input {
    data: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            data: vec![197, 130, 1],
        },
        Input {
            data: vec![235, 140, 4],
        },
        Input {
            data: vec![240, 162, 138, 147],
        },
    ];

    for Input { data } in inputs.into_iter() {
        let result = Solution::valid_utf8(data);
        println!("{result}");
    }
}
