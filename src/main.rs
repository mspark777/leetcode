mod utils;

use utils::Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();

        let mut left = 0;
        let mut right = (people.len() as i32) - 1;
        let mut result = 0;
        while left <= right {
            let light = people[left as usize];
            let heavy = people[right as usize];
            let total = light + heavy;

            result += 1;
            right -= 1;
            if total <= limit {
                left += 1;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 2], 3),
        (vec![3, 2, 2, 1], 3),
        (vec![3, 5, 3, 4], 5),
    ];

    for (people, limit) in inputs {
        let result = Solution::num_rescue_boats(people, limit);
        println!("{result}");
    }
}
