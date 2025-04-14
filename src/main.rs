struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        for (i, ni) in arr.iter().cloned().enumerate() {
            for (j, nj) in arr.iter().cloned().enumerate().skip(i + 1) {
                for nk in arr.iter().cloned().skip(j + 1) {
                    if (ni - nj).abs() <= a && (nj - nk).abs() <= b && (ni - nk).abs() <= c {
                        result += 1;
                    }
                }
            }
        }
        return result;
    }
}

struct Input {
    arr: Vec<i32>,
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![3, 0, 1, 1, 9, 7],
            a: 7,
            b: 2,
            c: 3,
        },
        Input {
            arr: vec![1, 1, 2, 2, 3],
            a: 0,
            b: 0,
            c: 1,
        },
    ];

    for input in inputs {
        let result = Solution::count_good_triplets(input.arr, input.a, input.b, input.c);
        println!("{result:?}");
    }
}
