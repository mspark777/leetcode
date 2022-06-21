use std::cmp::Ordering;

#[allow(dead_code)]
pub struct MaxEnvelopes {}

#[allow(dead_code)]
impl MaxEnvelopes {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut clone = envelopes.clone();
        clone.sort_by(Self::compare_envelopes);

        let mut result: i32 = 0;
        let mut dp = vec![0i32; envelopes.len()];
        for envelope in clone {
            let height = envelope[1];
            let mut index = Self::search(&dp, 0, result as usize, height);
            if index < 0 {
                index = -(index + 1);
            }

            dp[index as usize] = height;
            if result == index {
                result += 1;
            }
        }
        result
    }

    fn compare_envelopes(left: &Vec<i32>, right: &Vec<i32>) -> Ordering {
        let wdiff = left[0] - right[0];
        if wdiff < 0 {
            return Ordering::Less;
        } else if wdiff > 0 {
            return Ordering::Greater;
        }

        let hdiff = right[1] - left[1];
        if hdiff < 0 {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }

    fn search(arr: &Vec<i32>, mut begin: usize, mut end: usize, value: i32) -> i32 {
        while begin < end {
            let mindex = (begin + end) / 2;
            let mvalue = arr[mindex];
            if mvalue < value {
                begin = mindex + 1;
            } else if mvalue > value {
                end = mindex;
            } else {
                return mindex as i32;
            }
        }

        -((begin + 1) as i32)
    }
}

#[allow(dead_code)]
pub struct MaxEnvelopes1 {}

#[allow(dead_code)]
impl MaxEnvelopes1 {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut clone = envelopes;
        clone.sort_by(Self::compare_envelopes);

        let mut result: i32 = 0;
        let mut dp = vec![0i32; clone.len()];
        for envelope in clone {
            let height = envelope[1];
            let mut index = Self::search(&dp, 0, result as usize, height);
            if index < 0 {
                index = -(index + 1);
            }

            dp[index as usize] = height;
            if result == index {
                result += 1;
            }
        }
        result
    }

    fn compare_envelopes(left: &Vec<i32>, right: &Vec<i32>) -> Ordering {
        let wdiff = left[0] - right[0];
        if wdiff < 0 {
            return Ordering::Less;
        } else if wdiff > 0 {
            return Ordering::Greater;
        }

        let hdiff = right[1] - left[1];
        if hdiff < 0 {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }

    fn search(arr: &Vec<i32>, mut begin: usize, mut end: usize, value: i32) -> i32 {
        while begin < end {
            let mindex = (begin + end) / 2;
            let mvalue = arr[mindex];
            if mvalue < value {
                begin = mindex + 1;
            } else if mvalue > value {
                end = mindex;
            } else {
                return mindex as i32;
            }
        }

        -((begin + 1) as i32)
    }
}

#[allow(dead_code)]
pub struct MaxEnvelopes2 {}

#[allow(dead_code)]
impl MaxEnvelopes2 {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut clone = envelopes;
        clone.sort_unstable_by(Self::compare_envelopes);

        let mut dp = vec![];
        for envelope in clone {
            let height = envelope[1];
            if let Err(p) = dp.binary_search(&height) {
                if p == dp.len() {
                    dp.push(height);
                } else {
                    dp[p] = height;
                }
            }
        }
        dp.len() as i32
    }

    fn compare_envelopes(left: &Vec<i32>, right: &Vec<i32>) -> Ordering {
        let wdiff = left[0] - right[0];
        if wdiff < 0 {
            return Ordering::Less;
        } else if wdiff > 0 {
            return Ordering::Greater;
        }

        let hdiff = right[1] - left[1];
        if hdiff < 0 {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}
