#[allow(dead_code)]
pub struct SingleNumber {}

#[allow(dead_code)]
impl SingleNumber {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for i in nums {
            result ^= i;
        }

        result
    }
}

#[allow(dead_code)]
pub struct SingleNumber1 {}

#[allow(dead_code)]
impl SingleNumber1 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0i32, |acc, n| acc ^ n)
    }
}
