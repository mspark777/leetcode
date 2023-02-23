use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = capital
            .iter()
            .cloned()
            .zip(profits.iter().cloned())
            .collect();

        projects.sort_unstable();

        let mut result = w;
        let mut queue = BinaryHeap::<i32>::with_capacity(projects.len());
        let mut pro = 0usize;

        for _ in 0..k {
            while let Some(project) = projects.get(pro) {
                if project.0 <= result {
                    queue.push(project.1);
                    pro += 1;
                } else {
                    break;
                }
            }

            if let Some(p) = queue.pop() {
                result += p;
            } else {
                break;
            }
        }

        return result;
    }
}

struct Input {
    k: i32,
    w: i32,
    profits: Vec<i32>,
    capital: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            k: 2,
            w: 0,
            profits: vec![1, 2, 3],
            capital: vec![0, 1, 1],
        },
        Input {
            k: 3,
            w: 0,
            profits: vec![1, 2, 3],
            capital: vec![0, 1, 2],
        },
    ];

    for Input {
        k,
        w,
        profits,
        capital,
    } in inputs
    {
        let result = Solution::find_maximized_capital(k, w, profits, capital);
        println!("{result}");
    }
}
