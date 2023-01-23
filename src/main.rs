struct Solution {}
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; n as usize];
        for info in trust.iter() {
            let from = info[0] - 1;
            let to = info[1] - 1;

            counts[from as usize] -= 1;
            counts[to as usize] += 1;
        }

        let judge = n - 1;
        for (person, &count) in counts.iter().enumerate() {
            if count == judge {
                let p = person + 1;
                return p as i32;
            }
        }

        return -1;
    }
}

struct Input {
    n: i32,
    trust: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 2,
            trust: vec![vec![1, 2]],
        },
        Input {
            n: 3,
            trust: vec![vec![1, 3], vec![2, 3]],
        },
        Input {
            n: 3,
            trust: vec![vec![1, 3], vec![2, 3], vec![3, 1]],
        },
        Input {
            n: 1,
            trust: vec![],
        },
    ];

    for Input { n, trust } in inputs {
        let result = Solution::find_judge(n, trust);
        println!("{result}");
    }
}
