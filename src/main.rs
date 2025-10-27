struct Solution {}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.chars().collect::<Vec<char>>();
        let k = k as usize;
        let mut left = 0usize;
        let mut whites = 0;
        let mut result = i32::MAX;

        for (right, color) in blocks.iter().cloned().enumerate() {
            if color == 'W' {
                whites += 1;
            }

            let len = right + 1 - left;
            if len == k {
                result = result.min(whites);
                if blocks[left] == 'W' {
                    whites -= 1;
                }

                left += 1;
            }
        }

        result
    }
}

struct Input {
    blocks: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            blocks: "WBBWWBBWBW".to_string(),
            k: 7,
        },
        Input {
            blocks: "WBWBBBW".to_string(),
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_recolors(input.blocks, input.k);
        println!("{:?}", result);
    }
}
