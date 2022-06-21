#[allow(dead_code)]
pub struct NumberOfSteps {}

#[allow(dead_code)]
impl NumberOfSteps {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num as u32;
        let mut result = -1;

        while n > 0 {
            result += if (n & 1) == 1 { 2 } else { 1 };

            n >>= 1;
        }

        result.max(0)
    }
}
