struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut frequencies = std::collections::HashMap::<i32, i32>::new();
        for num in arr.iter().cloned() {
            frequencies.entry(num).and_modify(|f| *f += 1).or_insert(1);
        }

        frequencies.iter().fold(-1, |acc, (&k, &v)| match k == v {
            true => acc.max(k),
            _ => acc,
        })
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![2, 2, 3, 4],
        },
        Input {
            arr: vec![1, 2, 2, 3, 3, 3],
        },
        Input {
            arr: vec![2, 2, 2, 3, 3],
        },
    ];

    for input in inputs {
        let result = Solution::find_lucky(input.arr);
        println!("{:?}", result);
    }
}
