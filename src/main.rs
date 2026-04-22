struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        use std::collections::BinaryHeap;

        let n = nums.len();
        let mut queue = BinaryHeap::<i32>::from_iter(nums.iter().copied());

        let mut i = 1usize;
        while let Some(&num) = queue.peek() {
            if i >= n {
                break;
            }

            nums[i] = num;
            i += 2;
            queue.pop();
        }

        i = 0usize;
        while let Some(&num) = queue.peek() {
            if i >= n {
                break;
            }

            nums[i] = num;
            i += 2;
            queue.pop();
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 5, 1, 1, 6, 4].to_vec(),
        },
        Input {
            nums: [1, 3, 2, 2, 3, 1].to_vec(),
        },
    ];

    for mut input in inputs.into_iter() {
        Solution::wiggle_sort(&mut input.nums);
        println!("{:?}", input.nums);
    }
}
