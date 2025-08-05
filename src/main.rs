struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut baskets = baskets;
        let mut result = 0;
        for fruit in fruits.iter().cloned() {
            let mut unset = 1;
            for capacity in baskets.iter_mut() {
                if fruit <= *capacity {
                    *capacity = 0;
                    unset = 0;
                    break;
                }
            }
            result += unset;
        }

        result
    }
}

struct Input {
    fruits: Vec<i32>,
    baskets: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            fruits: [4, 2, 5].to_vec(),
            baskets: [3, 5, 4].to_vec(),
        },
        Input {
            fruits: [3, 6, 1].to_vec(),
            baskets: [6, 4, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::num_of_unplaced_fruits(input.fruits, input.baskets);
        println!("{:?}", result);
    }
}
