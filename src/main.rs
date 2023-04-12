mod utils;

use utils::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let segments = path.split("/");
        let mut result = Vec::<&str>::new();

        for seg in segments {
            if seg.eq("") {
                continue;
            } else if seg.eq(".") {
                continue;
            } else if seg.eq("..") {
                result.pop();
            } else {
                result.push(seg);
            }
        }

        return format!("/{}", result.join("/"));
    }
}

fn main() {
    let inputs = ["/home/", "/../", "/home//foo/"];

    for path in inputs {
        let result = Solution::simplify_path(path.to_string());
        println!("{result}");
    }
}
