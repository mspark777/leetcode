struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut last1 = nums[0];
        let mut last2 = nums[1];
        let mut arr1 = vec![last1];
        let mut arr2 = vec![last2];

        for num in nums.iter().skip(2).copied() {
            if last1 > last2 {
                arr1.push(num);
                last1 = num;
            } else {
                arr2.push(num);
                last2 = num;
            }
        }

        arr1.append(&mut arr2);
        arr1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 1, 3].to_vec(),
        },
        Input {
            nums: [5, 4, 3, 8].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::result_array(input.nums);
        println!("{:?}", result);
    }
}
