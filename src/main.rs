struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len() + 1];
        for (i, &num) in nums.iter().enumerate() {
            sums[i + 1] += sums[i] + num;
        }

        return NumArray { sums };
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let sums = &self.sums;
        let left = left as usize;
        let right = right as usize;

        return sums[right + 1] - sums[left];
    }
}

fn main() {
    let obj = NumArray::new([-2, 0, 3, -5, 2, -1].to_vec());
    println!("{}", obj.sum_range(0, 2));
    println!("{}", obj.sum_range(2, 5));
    println!("{}", obj.sum_range(0, 5));
}
