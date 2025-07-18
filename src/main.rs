struct Solution {}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut possible_dups = 0usize;
        let mut n = arr.len() - 1;
        let mut left = 0usize;
        while left <= (n - possible_dups) {
            if arr[left] == 0 {
                if left == (n - possible_dups) {
                    arr[n] = 0;
                    n -= 1;
                    break;
                }
                possible_dups += 1;
            }

            left += 1;
        }

        for i in (0..=(n - possible_dups)).rev() {
            if arr[i] == 0 {
                arr[i + possible_dups] = 0;
                possible_dups -= 1;
                arr[i + possible_dups] = 0;
            } else {
                arr[i + possible_dups] = arr[i];
            }
        }
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: [1, 0, 2, 3, 0, 4, 5, 0].to_vec(),
        },
        Input {
            arr: [1, 2, 3].to_vec(),
        },
    ];

    for input in inputs {
        let mut input = input;
        Solution::duplicate_zeros(&mut input.arr);
        println!("{:?}", input.arr);
    }
}
