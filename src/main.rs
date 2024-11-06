struct Solution {}

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut cur_bits = 0;
        let mut prev_max_num = 0;
        let mut max_num = 0;

        for &num in nums.iter() {
            let bits = num.count_ones();
            if bits != cur_bits {
                cur_bits = bits;
                prev_max_num = max_num;
            }

            if num < prev_max_num {
                return false;
            }

            max_num = max_num.max(num);
        }

        return true;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![8, 4, 2, 30, 15],
        },
        Input {
            nums: vec![1, 2, 3, 4, 5],
        },
        Input {
            nums: vec![3, 16, 8, 4, 2],
        },
    ];

    for input in inputs {
        let result = Solution::can_sort_array(input.nums);
        println!("{result}");
    }
}
