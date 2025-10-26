struct Solution {}

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let num_set = std::collections::HashSet::<i32>::from_iter(nums.iter().cloned());
        let mut result = 0;
        let last = *nums.last().unwrap();

        for num in nums.iter().cloned() {
            let j_num = num + diff;
            let k_num = j_num + diff;
            if k_num > last {
                continue;
            } else if j_num > last {
                break;
            }

            if num_set.contains(&j_num) && num_set.contains(&k_num) {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    dif: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [0, 1, 4, 6, 7, 10].to_vec(),
            dif: 3,
        },
        Input {
            nums: [4, 5, 6, 7, 8, 9].to_vec(),
            dif: 2,
        },
    ];

    for input in inputs {
        let result = Solution::arithmetic_triplets(input.nums, input.dif);
        println!("{:?}", result);
    }
}
