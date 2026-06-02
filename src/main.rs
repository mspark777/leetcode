struct Solution;

fn not_subseq(a: &str, b: &str) -> bool {
    let mut it = b.chars();
    a.chars().any(|c| it.all(|x| x != c))
}

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        (0..strs.len())
            .filter(|&i| (0..strs.len()).all(|j| i == j || not_subseq(&strs[i], &strs[j])))
            .map(|i| strs[i].len() as i32)
            .max()
            .unwrap_or(-1)
    }
}

struct Input {
    strs: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            strs: ["aba", "cdc", "eae"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            strs: ["aaa", "aaa", "aa"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            strs: ["aabbcc", "aabbcc", "c"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_lu_slength(input.strs);
        println!("{:?}", result);
    }
}
