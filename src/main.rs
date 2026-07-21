struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lengths = vec![1; n];
        let mut counts = vec![1; n];

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if (lengths[j] + 1) > lengths[i] {
                        lengths[i] = lengths[j] + 1;
                        counts[i] = 0;
                    }
                    if (lengths[j] + 1) == lengths[i] {
                        counts[i] += counts[j];
                    }
                }
            }
        }

        let max_length = lengths.iter().copied().max().unwrap();
        let mut result = 0;
        for (i, l) in lengths.into_iter().enumerate() {
            if l == max_length {
                result += counts[i];
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [1, 3, 5, 4, 7].to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_number_of_lis(input.nums);
        println!("{:?}", result);
    }
}
