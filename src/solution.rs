use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let bs = s.as_bytes();
        let mut counter = HashMap::<u8, i32>::with_capacity(bs.len());

        for v in bs {
            if let Some(count) = counter.get_mut(v) {
                *count += 1;
            } else {
                counter.insert(*v, 1);
            }
        }

        for v in t.as_bytes() {
            if let Some(count) = counter.get_mut(v) {
                if *count == 1 {
                    counter.remove(v);
                } else {
                    *count -= 1;
                }
            } else {
                return false;
            }
        }

        counter.len() < 1
    }
}
