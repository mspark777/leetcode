struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = Vec::<i32>::with_capacity(nums.len());

        for rnum in nums.iter() {
            let num = *rnum;
            match result.binary_search(rnum) {
                Ok(i) => {
                    if num < result[i] {
                        result[i] = num;
                    }
                }
                Err(i) => {
                    if i == result.len() {
                        result.push(num);
                    } else if num < result[i] {
                        result[i] = num;
                    }
                }
            }
        }

        result.len() as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![10, 9, 2, 5, 3, 7, 101, 18],
        },
        Input {
            nums: vec![0, 1, 0, 3, 2, 3],
        },
        Input {
            nums: vec![7, 7, 7, 7, 7, 7, 7],
        },
    ];

    for input in inputs {
        let nums = input.nums;
        let result = Solution::length_of_lis(nums);
        println!("{:?}", result);
    }
}
