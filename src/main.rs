use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut path_map = HashMap::<&str, Vec<String>>::with_capacity(paths.len());
        for path in paths.iter() {
            let mut segments = path.split(" ").into_iter();
            let root = segments.next().unwrap();
            for file in segments {
                let sep = file.find("(").unwrap();
                let name = &file[0..sep];
                let content = &file[sep..file.len() - 1];

                let filename = format!("{root}/{name}");
                if let Some(list) = path_map.get_mut(content) {
                    list.push(filename);
                } else {
                    path_map.insert(content, vec![filename]);
                }
            }
        }

        let mut result: Vec<Vec<String>> = Vec::with_capacity(path_map.len());

        for value in path_map.into_values() {
            if value.len() > 1 {
                result.push(value)
            }
        }

        result
    }
}

struct Input {
    paths: Vec<&'static str>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            paths: vec![
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
                "root 4.txt(efgh)",
            ],
        },
        Input {
            paths: vec![
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
            ],
        },
        Input {
            paths: vec![
                "root/a 1.txt(abcd) 2.txt(efsfgh)",
                "root/c 3.txt(abdfcd)",
                "root/c/d 4.txt(efggdfh)",
            ],
        },
    ];

    for Input { paths } in inputs.into_iter() {
        let result = Solution::find_duplicate(paths.iter().map(|s| s.to_string()).collect());
        println!("{result:?}");
    }
}
