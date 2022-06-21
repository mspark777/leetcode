#[allow(dead_code)]
pub struct HammingWeight {}

#[allow(dead_code, non_snake_case)]
impl HammingWeight {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut u = n;
        let mut count = 0;
        while u != 0 {
            u = u & (u - 1);
            count += 1;
        }
        count
    }

    pub fn hammingWeight1(n: u32) -> i32 {
        let mut u = n;
        let mut count = 0;
        while u != 0 {
            u &= u - 1;
            count += 1;
        }
        count
    }
}
