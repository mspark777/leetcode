struct Solution {}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut min = i32::max_value();
        let mut middle = i32::max_value();
        for n in nums.into_iter() {
            if n <= min {
                min = n;
            } else if n <= middle {
                middle = n;
            } else {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let inputs = [
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![2, 1, 5, 0, 4, 6],
        vec![2, 6, 1, 8],
    ];

    for nums in inputs {
        let result = Solution::increasing_triplet(nums);
        println!("{result}");
    }
}
