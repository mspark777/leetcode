#[allow(dead_code)]
pub struct TwoSum {}

#[allow(dead_code)]
impl TwoSum {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let ni = numbers[i];
            let nj = numbers[j];
            let ns = ni + nj;
            if ns < target {
                i += 1;
            } else if ns > target {
                j -= 1;
            } else {
                break;
            }
        }

        let head = (i + 1) as i32;
        let tail = (j + 1) as i32;
        vec![head, tail]
    }
}
