pub struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::<i32>::with_capacity(nums.len());

        for n in nums {
            if let Some(_) = set.get(&n) {
                return true;
            } else {
                set.insert(n);
            }
        }

        false
    }
}
