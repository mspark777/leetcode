struct Solution {}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort_unstable();

        let mut result = vec![folder[0].clone()];
        for f in folder.iter().skip(1) {
            let last = format!("{}/", result.last().unwrap());
            if !f.starts_with(last.as_str()) {
                result.push(f.clone());
            }
        }

        result
    }
}

struct Input {
    folder: Vec<String>,
}

fn main() {
    let inputs = vec![
        Input {
            folder: ["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
                .map(|s| s.to_string())
                .to_vec(),
        },
        Input {
            folder: ["/a", "/a/b/c", "/a/b/d"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            folder: ["/a/b/c", "/a/b/ca", "/a/b/d"]
                .map(|s| s.to_string())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::remove_subfolders(input.folder);
        println!("{:?}", result);
    }
}
