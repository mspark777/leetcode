struct Solution {}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for num in arr.iter().cloned() {
            if (num & 1) == 1 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        return false;
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![2, 6, 4, 1],
        },
        Input {
            arr: vec![1, 2, 34, 3, 4, 5, 7, 23, 12],
        },
    ];

    for input in inputs {
        let result = Solution::three_consecutive_odds(input.arr);
        println!("{result:?}");
    }
}
