struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut baskets = std::collections::HashMap::<i32, i32>::with_capacity(3);
        let mut left = 0usize;
        let mut result = 0usize;

        for (right, rfruit) in fruits.iter().cloned().enumerate() {
            baskets.entry(rfruit).and_modify(|f| *f += 1).or_insert(1);

            while baskets.len() > 2 {
                let lfruit = fruits[left];
                let lcount = baskets.get_mut(&lfruit).unwrap();
                if *lcount > 1 {
                    *lcount -= 1;
                } else {
                    baskets.remove(&lfruit);
                }

                left += 1;
            }

            result = result.max(right + 1 - left);
        }

        result as i32
    }
}

struct Input {
    fruits: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            fruits: [1, 2, 3].to_vec(),
        },
        Input {
            fruits: [0, 1, 2, 2].to_vec(),
        },
        Input {
            fruits: [1, 2, 3, 2, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::total_fruit(input.fruits);
        println!("{:?}", result);
    }
}
