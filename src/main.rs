struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut left = 0usize;
        let mut right = arr.len() - 1;

        let last = right;
        while (left < last) && (arr[left] < arr[left + 1]) {
            left += 1;
        }

        while (right > 0) && (arr[right - 1] > arr[right]) {
            right -= 1;
        }

        return (left > 0) && (left == right) && (right < last);
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input { arr: vec![2, 1] },
        Input { arr: vec![3, 5, 5] },
        Input {
            arr: vec![0, 3, 2, 1],
        },
    ];

    for input in inputs {
        let result = Solution::valid_mountain_array(input.arr);
        println!("{result:?}");
    }
}
