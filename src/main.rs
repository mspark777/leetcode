struct Solution {}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut buf0 = nums.clone();
        let mut buf1 = nums.clone();
        let mut n = nums.len();
        let mut write = &mut buf0;
        let mut read = &mut buf1;

        while n > 1 {
            n /= 2;
            for i in 0..n {
                if i & 1 == 1 {
                    write[i] = read[i * 2].max(read[i * 2 + 1]);
                } else {
                    write[i] = read[i * 2].min(read[i * 2 + 1]);
                }
            }

            let temp = write;
            write = read;
            read = temp;
        }

        read[0]
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 3, 5, 2, 4, 8, 2, 2].to_vec(),
        },
        Input { nums: [3].to_vec() },
        Input {
            nums: [93, 40].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_max_game(input.nums);
        println!("{:?}", result);
    }
}
