struct Solution {}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len();
        let mut left = 0usize;
        let mut right = 0usize;
        let mut sum = 0;
        let mut result = 0;

        while right < n {
            sum += fruits[right][1];
            while (left <= right) && (Self::step(&fruits, start_pos, left, right) > k) {
                sum -= fruits[left][1];
                left += 1;
            }

            result = result.max(sum);
            right += 1;
        }

        result
    }

    fn step(fruits: &Vec<Vec<i32>>, start_pos: i32, left: usize, right: usize) -> i32 {
        let l = fruits[left][0];
        let r = fruits[right][0];
        (start_pos - r).abs().min((start_pos - l).abs()) + r - l
    }
}

struct Input {
    fruits: Vec<Vec<i32>>,
    start_pos: i32,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            fruits: [[2, 8], [6, 3], [8, 6]].map(|f| f.to_vec()).to_vec(),
            start_pos: 5,
            k: 4,
        },
        Input {
            fruits: [[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]]
                .map(|f| f.to_vec())
                .to_vec(),
            start_pos: 5,
            k: 4,
        },
        Input {
            fruits: [[0, 3], [6, 4], [8, 5]].map(|f| f.to_vec()).to_vec(),
            start_pos: 3,
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::max_total_fruits(input.fruits, input.start_pos, input.k);
        println!("{:?}", result);
    }
}
