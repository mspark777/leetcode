struct Solution {}
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = 0;
        let mut result = 0;

        let mut mod_groups = vec![0; k as usize];
        mod_groups[0] = 1;

        for &num in nums.iter() {
            prefix = (prefix + k + (num % k)) % k;

            let i = prefix as usize;
            result += mod_groups[i];
            mod_groups[i] += 1;
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![4, 5, 0, -2, -3, 1],
            k: 5,
        },
        Input {
            nums: vec![5],
            k: 9,
        },
    ];

    for Input { nums, k } in inputs {
        let result = Solution::subarrays_div_by_k(nums, k);
        println!("{result}");
    }
}
