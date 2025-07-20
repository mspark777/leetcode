use std::collections::HashMap;

struct Solution {}

#[derive(Default)]
struct Trie {
    serial: String,
    children: HashMap<String, Trie>,
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::default();
        for path in paths.iter() {
            let mut cur = &mut root;
            for node in path.iter() {
                cur = cur.children.entry(node.clone()).or_default();
            }
        }

        let mut freq = HashMap::<String, usize>::new();
        Self::construct(&mut root, &mut freq);
        let mut result = Vec::<Vec<String>>::new();
        let mut path = Vec::<String>::new();
        Self::operate(&root, &freq, &mut path, &mut result);

        result
    }

    fn construct(node: &mut Trie, freq: &mut HashMap<String, usize>) {
        if node.children.is_empty() {
            return;
        }

        let mut v = Vec::new();
        for (folder, child) in node.children.iter_mut() {
            Self::construct(child, freq);
            v.push(format!("{}({})", folder, child.serial));
        }

        v.sort();
        node.serial = v.join("");
        *freq.entry(node.serial.clone()).or_default() += 1;
    }

    fn operate(
        node: &Trie,
        freq: &HashMap<String, usize>,
        path: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if freq.get(&node.serial).unwrap_or(&0) > &1 {
            return;
        }

        if !path.is_empty() {
            ans.push(path.clone());
        }

        for (folder, child) in node.children.iter() {
            path.push(folder.clone());
            Self::operate(child, freq, path, ans);
            path.pop();
        }
    }
}

struct Input {
    paths: Vec<Vec<String>>,
}

fn main() {
    let inputs = vec![
        Input {
            paths: [
                vec!["a"],
                vec!["c"],
                vec!["d"],
                vec!["a", "b"],
                vec!["c", "b"],
                vec!["d", "a"],
            ]
            .map(|path| path.iter().map(|p| p.to_string()).collect())
            .to_vec(),
        },
        Input {
            paths: [
                vec!["a"],
                vec!["c"],
                vec!["a", "b"],
                vec!["c", "b"],
                vec!["a", "b", "x"],
                vec!["a", "b", "x", "y"],
                vec!["w"],
                vec!["w", "y"],
            ]
            .map(|path| path.iter().map(|p| p.to_string()).collect())
            .to_vec(),
        },
        Input {
            paths: [vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]]
                .map(|path| path.iter().map(|p| p.to_string()).collect())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::delete_duplicate_folder(input.paths);
        println!("{:?}", result);
    }
}
