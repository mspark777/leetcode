pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut result = vec![1; row_index + 1];

        for i in 0..=row_index {
            for j in (1..i).rev() {
                result[j] += result[j - 1];
            }
        }

        result
    }
}
