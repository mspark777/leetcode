struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::<i32>::new();
        let mut r = 0usize;
        for (j, num) in nums.iter().cloned().enumerate() {
            if num == key {
                let l = if j > k { r.max(j - k) } else { r };
                r = (nums.len() - 1).min(j + k) + 1;
                for i in l..r {
                    result.push(i as i32);
                }
            }
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    key: i32,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: [3, 4, 9, 1, 3, 9, 5].to_vec(),
            key: 9,
            k: 1,
        },
        Input {
            nums: [2, 2, 2, 2, 2].to_vec(),
            key: 2,
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::find_k_distant_indices(input.nums, input.key, input.k);
        println!("{:?}", result);
    }
}
