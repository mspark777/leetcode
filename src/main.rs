struct Solution {}

impl Solution {
    pub fn find_the_prefix_common_array(a_list: Vec<i32>, b_list: Vec<i32>) -> Vec<i32> {
        let n = a_list.len();
        let mut result = vec![0; n];
        let mut frequencies = vec![0; n];
        let mut common_count = 0;

        for (i, (&a, &b)) in a_list.iter().zip(b_list.iter()).enumerate() {
            let a_idx = (a as usize) - 1;
            let b_idx = (b as usize) - 1;

            frequencies[a_idx] += 1;
            if frequencies[a_idx] == 2 {
                common_count += 1;
            }

            frequencies[b_idx] += 1;
            if frequencies[b_idx] == 2 {
                common_count += 1;
            }

            result[i] = common_count;
        }

        return result;
    }
}

struct Input {
    a: Vec<i32>,
    b: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            a: vec![1, 3, 2, 4],
            b: vec![3, 1, 2, 4],
        },
        Input {
            a: vec![2, 3, 1],
            b: vec![3, 1, 2],
        },
    ];

    for input in inputs {
        let result = Solution::find_the_prefix_common_array(input.a, input.b);
        println!("{result:?}");
    }
}
