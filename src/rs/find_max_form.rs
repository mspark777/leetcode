#[allow(dead_code)]
pub struct FindMaxForm {}

#[allow(dead_code)]
impl FindMaxForm {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut maxs = vec![0i32; ((m + 1) * (n + 1)) as usize];
        for s in strs.iter() {
            let mut one: i32 = 0;
            let mut zero: i32 = 0;

            for c in s.chars() {
                if c == '0' {
                    zero += 1i32;
                } else {
                    one += 1i32;
                }
            }
            for i in (zero..=m).rev() {
                for j in (one..=n).rev() {
                    let m0index = (i * (n + 1) + j) as usize;
                    let m1index = ((i - zero) * (n + 1) + (j - one)) as usize;
                    let m0 = maxs[m0index];
                    let m1 = maxs[m1index] + 1;
                    if m1 > m0 {
                        maxs[m0index] = m1;
                    }
                }
            }
        }

        let last_index = (m + 1) * (n + 1) - 1;
        maxs[last_index as usize]
    }
}
