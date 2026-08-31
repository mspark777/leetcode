struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for path in paths {
            let (x, y) = (path[0] as usize - 1, path[1] as usize - 1);
            graph[x].push(y);
            graph[y].push(x);
        }

        let mut colors = vec![0; n];
        for i in 0..n {
            let mut used = vec![false; 5];
            for j in graph[i].iter().copied() {
                used[colors[j] as usize] = true;
            }
            for c in 1..=4 {
                if !used[c] {
                    colors[i] = c as i32;
                    break;
                }
            }
        }
        colors
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
