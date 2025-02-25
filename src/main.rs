struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prefix_sum = 0;
        let mut odd_count = 0;
        let mut even_count = 1;

        for num in arr.iter().cloned() {
            prefix_sum += num;

            if prefix_sum & 1 == 0 {
                result += odd_count;
                even_count += 1;
            } else {
                result += even_count;
                odd_count += 1;
            }

            result %= 1_000_000_007;
        }

        return result;
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input { arr: vec![1, 3, 5] },
        Input { arr: vec![2, 4, 6] },
        Input {
            arr: vec![1, 2, 3, 4, 5, 6, 7],
        },
    ];

    for input in inputs {
        let result = Solution::num_of_subarrays(input.arr);
        println!("{result:?}");
    }
}
