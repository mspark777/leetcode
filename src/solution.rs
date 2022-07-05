pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut longest = 0;
        let mut num_set = HashSet::<i32>::with_capacity(nums.len());
        for n in nums {
            num_set.insert(n);
        }

        for num in num_set.iter() {
            let prev = num - 1;
            if num_set.contains(&prev) {
                continue;
            }

            let mut cur = num + 1;
            let mut count = 1;
            while num_set.contains(&cur) {
                count += 1;
                cur += 1;
            }

            longest = longest.max(count);
        }

        longest
    }
}
