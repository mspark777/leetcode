struct Solution {}

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums_sorted = nums.clone();
        nums_sorted.sort_unstable();

        let mut curr_group = 0;
        let mut num_to_group = std::collections::HashMap::<i32, i32>::new();
        num_to_group.insert(nums_sorted[0], curr_group);

        let mut group_to_list =
            std::collections::HashMap::<i32, std::collections::VecDeque<i32>>::new();
        group_to_list.insert(
            curr_group,
            std::collections::VecDeque::from([nums_sorted[0]]),
        );

        for i in 1..nums.len() {
            let diff = nums_sorted[i] - nums_sorted[i - 1];
            if diff.abs() > limit {
                curr_group += 1;
            }

            num_to_group.insert(nums_sorted[i], curr_group);
            if !group_to_list.contains_key(&curr_group) {
                group_to_list.insert(curr_group, std::collections::VecDeque::new());
            }

            group_to_list
                .get_mut(&curr_group)
                .unwrap()
                .push_back(nums_sorted[i]);
        }

        let mut result = nums.clone();
        for num in result.iter_mut() {
            let group_no = num_to_group.get(num).unwrap();
            *num = group_to_list
                .get_mut(group_no)
                .unwrap()
                .pop_front()
                .unwrap();
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    limit: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 5, 3, 9, 8],
            limit: 2,
        },
        Input {
            nums: vec![1, 7, 6, 18, 2, 1],
            limit: 3,
        },
        Input {
            nums: vec![1, 7, 28, 19, 10],
            limit: 3,
        },
    ];

    for input in inputs {
        let result = Solution::lexicographically_smallest_array(input.nums, input.limit);
        println!("{result:?}");
    }
}
