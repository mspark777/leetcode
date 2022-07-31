pub struct NumArray {
    tree: Vec<i32>,
    len: usize,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        return NumArray {
            tree: Self::build_tree(&nums),
            len: nums.len(),
        };
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let mut index = (index as usize) + self.len;
        let tree = &mut self.tree;

        tree[index] = val;
        while index > 0 {
            let mut left = index;
            let mut right = index;
            if index % 2 == 0 {
                right = index + 1;
            } else {
                left = index - 1;
            }

            index /= 2;
            tree[index] = tree[left] + tree[right];
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut left = left + (self.len as i32);
        let mut right = right + (self.len as i32);
        let tree = &self.tree;

        let mut sum = 0;
        while left <= right {
            if left % 2 == 1 {
                sum += tree[left as usize];
                left += 1;
            }

            if right % 2 == 0 {
                sum += tree[right as usize];
                right -= 1;
            }

            left /= 2;
            right /= 2;
        }

        sum
    }

    fn build_tree(nums: &Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut tree = vec![0; len * 2];
        for i in 0..len {
            tree[i + len] = nums[i];
        }

        for i in (0..len).rev() {
            tree[i] = tree[i * 2] + tree[i * 2 + 1];
        }

        tree
    }
}
