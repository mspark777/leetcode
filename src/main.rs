struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut start = Vec::<i32>::new();
        for num in nums.iter().copied() {
            match start.last() {
                Some(&s) if s < num => start.push(num),
                None => start.push(num),
                _ => break,
            };
        }

        let mut end = Vec::<i32>::new();
        for num in nums.iter().copied().rev() {
            match end.last() {
                Some(&e) if e > num => end.push(num),
                None => end.push(num),
                _ => break,
            };
        }

        if (start.len() + end.len()) > n {
            return (n * (n + 1) / 2) as i32;
        }

        let mut result = start.len() + end.len() + 1;
        let mut i = 0usize;
        let mut j = 0usize;

        end.reverse();
        while (i < start.len()) && (j < end.len()) {
            if start[i] < end[j] {
                result += end.len() - j;
                i += 1;
            } else {
                j += 1;
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
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [6, 5, 7, 8].to_vec(),
        },
        Input {
            nums: [8, 7, 6, 6].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::incremovable_subarray_count(input.nums);
        println!("{:?}", result);
    }
}
