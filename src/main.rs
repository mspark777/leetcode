struct Solution {}

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums_array = grid.iter().cloned().flatten().collect::<Vec<_>>();
        nums_array.sort_unstable();

        let final_common_number = nums_array[nums_array.len() / 2];
        let mut result = 0;
        for num in nums_array.iter().cloned() {
            if (num % x) != (final_common_number % x) {
                return -1;
            }

            result += (final_common_number - num).abs() / x;
        }

        return result;
    }
}

struct Input {
    x: i32,
    rectangles: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            x: 2,
            rectangles: vec![vec![2, 4], vec![6, 8]],
        },
        Input {
            x: 1,
            rectangles: vec![vec![1, 5], vec![2, 3]],
        },
        Input {
            x: 2,
            rectangles: vec![vec![1, 2], vec![3, 4]],
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.rectangles, input.x);
        println!("{result:?}");
    }
}
