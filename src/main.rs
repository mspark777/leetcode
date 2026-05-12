struct Solution;

impl Solution {
    pub fn find_maximum_xor(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();

        let nums = nums.into_iter().map(|n| n as u32).collect::<Vec<u32>>();
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut result = nums[0] ^ nums[n - 1];
        let mut upper = 1;
        while upper <= nums[n - 1] {
            upper <<= 1;
        }

        upper -= 1;
        while l + 1 < r {
            let a = nums[l];
            let b = nums[l + 1];
            let c = nums[r - 1];
            let d = nums[r];
            result = result.max(a ^ b).max(a ^ c).max(b ^ d).max(c ^ d);
            if a + d < upper {
                l += 1;
            } else {
                r -= 1;
            }
        }

        result as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 10, 5, 25, 2, 8].to_vec(),
        },
        Input {
            nums: [14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_maximum_xor(input.nums);
        println!("{:?}", result);
    }
}
