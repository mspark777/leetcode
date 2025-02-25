struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let total_sum = arr.iter().cloned().reduce(|acc, n| acc + n).unwrap();
        if total_sum % 3 != 0 {
            return false;
        }

        let sum_of_partition = total_sum / 3;
        let mut partitions = 0;
        let mut sum = 0;
        let mut pos = 0usize;

        for num in arr.iter().cloned() {
            if partitions >= 2 {
                break;
            }

            sum += num;
            if sum == sum_of_partition {
                partitions += 1;
                sum = 0;
            }
            pos += 1;
        }

        return partitions == 2 && pos != arr.len();
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            arr: vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1],
        },
        Input {
            arr: vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1],
        },
        Input {
            arr: vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4],
        },
        Input {
            arr: vec![1, -1, 1, -1],
        },
    ];

    for input in inputs {
        let result = Solution::can_three_parts_equal_sum(input.arr);
        println!("{result:?}");
    }
}
