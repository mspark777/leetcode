struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut visited = vec![false; n + 1];
        let mut result = 0;
        Self::calculate(n, 1, &mut visited, &mut result);

        result
    }

    fn calculate(n: usize, pos: usize, visited: &mut [bool], result: &mut i32) {
        if pos > n {
            *result += 1;
            return;
        }

        for i in 1..=n {
            if visited[i] {
                continue;
            } else if pos.is_multiple_of(i) || i.is_multiple_of(pos) {
                visited[i] = true;
                Self::calculate(n, pos + 1, visited, result);
                visited[i] = false;
            }
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 2 }, Input { n: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::count_arrangement(input.n);
        println!("{:?}", result);
    }
}
