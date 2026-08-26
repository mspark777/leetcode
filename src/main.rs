struct Solution;

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut a = a;
        let mut result: Vec<(i32, i32)> = Vec::new();
        Solution::helper(&mut a, 0, &mut result);

        result
            .into_iter()
            .max()
            .map(|max| format!("{:02}:{:02}", max.0, max.1))
            .unwrap_or_default()
    }
    fn helper(a: &mut [i32], i: usize, answers: &mut Vec<(i32, i32)>) {
        if i == 4 {
            let (h, m) = (a[0] * 10 + a[1], a[2] * 10 + a[3]);
            if h < 24 && m < 60 {
                answers.push((h, m));
            }
            return;
        }

        for j in i..4 {
            a.swap(i, j);
            Solution::helper(a, i + 1, answers);
            a.swap(i, j);
        }
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [1, 2, 3, 4].to_vec(),
    }];

    for input in inputs {
        let result = Solution::largest_time_from_digits(input.arr);
        println!("{:?}", result);
    }
}
