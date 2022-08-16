struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums {
            if count < 1 {
                candidate = num;
            }

            count += if num == candidate { 1 } else { -1 }
        }

        candidate
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![3, 2, 3],
        },
        Input {
            nums: vec![2, 2, 1, 1, 1, 2, 2],
        },
    ];

    for input in inputs {
        let result = Solution::majority_element(input.nums);
        println!("{:?}", result);
    }
}
