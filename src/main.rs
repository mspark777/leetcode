struct Solution {}
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by_key(|p| (-p[0], p[1]));

        let mut result = 0;
        let mut max_defence = 0;

        for property in properties.iter() {
            let defence = property[1];
            if max_defence > defence {
                result += 1;
            } else {
                max_defence = defence;
            }
        }

        result
    }
}
struct Input {
    properties: Vec<Vec<i32>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            properties: vec![vec![5, 5], vec![6, 3], vec![3, 6]],
        },
        Input {
            properties: vec![vec![2, 2], vec![3, 3]],
        },
        Input {
            properties: vec![vec![1, 5], vec![10, 4], vec![4, 3]],
        },
    ];

    for input in inputs.iter() {
        let properties = input.properties.clone();
        let result = Solution::number_of_weak_characters(properties);
        println!("{result:?}");
    }
}
