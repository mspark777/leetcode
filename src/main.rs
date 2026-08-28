struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return arr.len() as i32;
        }

        let n = arr.len();
        let last = n - 1;
        let mut best = 1;
        let mut i = 0;
        while i < last {
            if arr[i + 1] != arr[i] {
                best = best.max(2)
            }

            let mut j = i + 1;
            while j < last {
                j += 1;

                let [a, b, c] = arr[(j - 2)..=j] else {
                    panic!()
                };

                if (a < b && b > c) || (a > b && b < c) {
                    best = best.max(j - i + 1)
                } else {
                    i = j - 2;
                    break;
                }
            }
            i += 1
        }

        best as i32
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
