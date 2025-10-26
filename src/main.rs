struct Solution {}

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut m =
            std::collections::HashMap::<i32, i32>::with_capacity(items1.len() + items2.len());

        for item in items1.iter() {
            let value = item[0];
            let weight = item[1];
            m.insert(value, weight);
        }

        for item in items2.iter() {
            let value = item[0];
            let weight = item[1];
            m.entry(value)
                .and_modify(|w| *w += weight)
                .or_insert(weight);
        }

        let mut result: Vec<Vec<i32>> = m.iter().map(|m| [*m.0, *m.1].to_vec()).collect();
        result.sort_unstable_by_key(|r| r[0]);
        result
    }
}

struct Input {
    items1: Vec<Vec<i32>>,
    items2: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            items1: [[1, 1], [4, 5], [3, 8]].map(|a| a.to_vec()).to_vec(),
            items2: [[3, 1], [1, 5]].map(|a| a.to_vec()).to_vec(),
        },
        Input {
            items1: [[1, 1], [3, 2], [2, 3]].map(|a| a.to_vec()).to_vec(),
            items2: [[2, 1], [3, 2], [1, 3]].map(|a| a.to_vec()).to_vec(),
        },
        Input {
            items1: [[1, 3], [2, 2]].map(|a| a.to_vec()).to_vec(),
            items2: [[7, 1], [2, 2], [1, 4]].map(|a| a.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::merge_similar_items(input.items1, input.items2);
        println!("{:?}", result);
    }
}
