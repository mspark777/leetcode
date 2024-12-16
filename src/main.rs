struct Solution {}
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut queue =
            std::collections::BinaryHeap::<std::cmp::Reverse<(i32, usize)>>::with_capacity(
                nums.len(),
            );

        let mut result = nums.clone();
        for (i, num) in nums.iter().enumerate() {
            queue.push(std::cmp::Reverse((*num, i)));
        }

        for _ in 0..k {
            if queue.is_empty() {
                break;
            }

            let std::cmp::Reverse((num, i)) = queue.pop().unwrap();
            let new_num = num * multiplier;
            result[i] = new_num;
            queue.push(std::cmp::Reverse((new_num, i)));
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
    multiplier: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 1, 3, 5, 6],
            k: 5,
            multiplier: 2,
        },
        Input {
            nums: vec![1, 2],
            k: 3,
            multiplier: 4,
        },
    ];

    for input in inputs {
        let result = Solution::get_final_state(input.nums, input.k, input.multiplier);
        println!("{result:?}");
    }
}
