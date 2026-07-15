struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let n = n as usize;
        let mut stack = Vec::<i32>::new();
        let mut result = vec![0; n];
        let mut s = logs[0].split(':').collect::<Vec<_>>();

        stack.push(s[0].parse().unwrap());
        let mut prev = s[2].parse::<i32>().unwrap();

        let mut i = 1;
        while i < logs.len() {
            s = logs[i].split(':').collect();
            if s[1] == "start" {
                if !stack.is_empty() {
                    result[stack.last().copied().unwrap() as usize] +=
                        s[2].parse::<i32>().unwrap() - prev;
                }

                stack.push(s[0].parse().unwrap());
                prev = s[2].parse::<i32>().unwrap();
            } else {
                result[stack.last().copied().unwrap() as usize] +=
                    s[2].parse::<i32>().unwrap() - prev + 1;
                stack.pop();
                prev = s[2].parse::<i32>().unwrap() + 1;
            }
            i += 1;
        }

        result
    }
}

struct Input {
    n: i32,
    logs: Vec<String>,
}

fn main() {
    let inputs = [Input {
        n: 2,
        logs: ["0:start:0", "1:start:2", "1:end:5", "0:end:6"]
            .map(|v| v.to_string())
            .to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::exclusive_time(input.n, input.logs);
        println!("{:?}", result);
    }
}
