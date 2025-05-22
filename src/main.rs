struct Solution {}

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut queries = queries;
        queries.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut heap = std::collections::BinaryHeap::<i32>::new();
        let mut delta_array = vec![0; nums.len() + 1];
        let mut operations = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            operations += delta_array[i];
            while j < queries.len() && queries[j][0] == i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }

            while operations < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                operations += 1;
                let end = heap.pop().unwrap() as usize;
                delta_array[end + 1] -= 1;
            }

            if operations < nums[i] {
                return -1;
            }
        }

        return heap.len() as i32;
    }
}

struct Input {
    nums: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 0, 2],
            queries: vec![vec![0, 2], vec![0, 2], vec![1, 1]],
        },
        Input {
            nums: vec![1, 1, 1, 1],
            queries: vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]],
        },
        Input {
            nums: vec![1, 2, 3, 4],
            queries: vec![vec![0, 3]],
        },
    ];

    for mut input in inputs {
        let result = Solution::max_removal(input.nums, input.queries);
        println!("{:?}", result);
    }
}
