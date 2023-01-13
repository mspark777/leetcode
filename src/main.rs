struct LongestPath {
    result: i32,
}

impl LongestPath {
    fn new() -> Self {
        return Self { result: 1 };
    }

    fn dfs(&mut self, current: usize, children: &Vec<Vec<usize>>, s: &[u8]) -> i32 {
        let mut longest_chain = 0;
        let mut second_longest_chain = 0;

        for &child in children[current].iter() {
            let child_longest_chain = self.dfs(child, children, s);
            if s[current] == s[child] {
                continue;
            }

            if child_longest_chain > longest_chain {
                second_longest_chain = longest_chain;
                longest_chain = child_longest_chain;
            } else if child_longest_chain > second_longest_chain {
                second_longest_chain = child_longest_chain;
            }
        }

        self.result = self.result.max(longest_chain + second_longest_chain + 1);
        return longest_chain + 1;
    }
}
struct Solution {}
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut children = vec![Vec::<usize>::new(); parent.len()];
        for i in 1..parent.len() {
            children[parent[i] as usize].push(i);
        }

        let mut solution = LongestPath::new();
        solution.dfs(0, &children, s.as_bytes());
        return solution.result;
    }
}

struct Input {
    parent: Vec<i32>,
    s: String,
}

fn main() {
    let inputs = [
        Input {
            parent: vec![-1, 0, 0, 1, 1, 2],
            s: "abacbe".to_string(),
        },
        Input {
            parent: vec![-1, 0, 0, 0],
            s: "aabc".to_string(),
        },
    ];

    for Input { parent, s } in inputs {
        let result = Solution::longest_path(parent, s);
        println!("{result}");
    }
}
