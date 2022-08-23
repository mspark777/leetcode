struct Solution {}
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::<String>::new();

        let mut i = 0;
        while i < nums.len() {
            let head = nums[i];
            loop {
                let j = i + 1;
                if j >= nums.len() {
                    break;
                }

                if (nums[i] + 1) != nums[j] {
                    break;
                }

                i = j;
            }

            let tail = nums[i];
            if head == tail {
                result.push(format!("{head}"));
            } else {
                result.push(format!("{head}->{tail}"));
            }
            i += 1;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![0, 1, 2, 4, 5, 7],
        },
        Input {
            nums: vec![0, 2, 3, 4, 6, 8, 9],
        },
    ];

    for input in inputs.iter() {
        let nums = &input.nums;
        let result = Solution::summary_ranges(nums.clone());
        println!("{:?}", result);
    }
}
