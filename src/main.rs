struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; 102];

        for line in nums {
            let start = line[0] as usize;
            let end = line[1] as usize;
            counts[start] += 1;
            counts[end + 1] -= 1;
        }

        let mut result = 0;
        let mut sum = 0;

        for count in counts.iter().skip(1).take(100).copied() {
            sum += count;
            if sum != 0 {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<[i32; 2]>,
}

fn main() {
    let inputs = [
        Input {
            nums: [[3, 6], [1, 5], [4, 7]].to_vec(),
        },
        Input {
            nums: [[1, 3], [5, 8]].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::number_of_points(input.nums.iter().map(|v| v.to_vec()).collect());
        println!("{:?}", result);
    }
}
