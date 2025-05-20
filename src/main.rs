struct Solution {}

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut delta_array = vec![0; nums.len() + 1];
        for query in queries.iter() {
            let left = query[0] as usize;
            let right = query[1] as usize;
            delta_array[left] += 1;
            delta_array[right + 1] -= 1;
        }

        let mut operation_counts = vec![0; delta_array.len()];
        let mut current_operations = 0;
        for (i, delta) in delta_array.iter().cloned().enumerate() {
            current_operations += delta;
            operation_counts[i] = current_operations;
        }

        for (num, operation_count) in nums.iter().cloned().zip(operation_counts.iter().cloned()) {
            if operation_count < num {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 0, 1],
            queries: vec![vec![0, 2]],
        },
        Input {
            nums: vec![4, 3, 2, 1],
            queries: vec![vec![1, 3], vec![0, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::is_zero_array(input.nums, input.queries);
        println!("{:?}", result);
    }
}
