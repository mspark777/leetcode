mod utils;
use utils::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut empty_left = true;
        let last = flowerbed.len() - 1;
        for (i, &plot) in flowerbed.iter().enumerate() {
            if plot == 1 {
                empty_left = false;
                continue;
            }

            let empty_right = (i == last) || (flowerbed[i + 1] == 0);
            if empty_left && empty_right {
                empty_left = false;
                count += 1;
                if count >= n {
                    return true;
                }
            } else {
                empty_left = true;
            }
        }

        return count >= n;
    }
}

struct Input {
    flowerbed: Vec<i32>,
    n: i32,
}

fn main() {
    let inputs = [
        Input {
            flowerbed: vec![1, 0, 0, 0, 1],
            n: 1,
        },
        Input {
            flowerbed: vec![1, 0, 0, 0, 1],
            n: 2,
        },
    ];

    for Input { flowerbed, n } in inputs {
        let result = Solution::can_place_flowers(flowerbed, n);
        println!("{result}")
    }
}
