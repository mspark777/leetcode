mod utils;

use std::collections::HashMap;

use utils::Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let mut dp = HashMap::<i32, HashMap<i32, f64>>::new();
        dp.insert(0, HashMap::new());
        dp.get_mut(&0).unwrap().insert(0, 0.5);

        let m = (n as f64 / 25.0).ceil() as i32;

        for k in 1..=m {
            dp.insert(k, HashMap::new());
            dp.get_mut(&0).unwrap().insert(k, 1.0);
            dp.get_mut(&k).unwrap().insert(0, 0.0);

            for j in 1..=k {
                let jk = Self::calculate_dp(j, k, &dp);
                let kj = Self::calculate_dp(k, j, &dp);

                dp.get_mut(&j).unwrap().insert(k, jk);
                dp.get_mut(&k).unwrap().insert(j, kj);
            }

            let kk = dp.get(&k).unwrap().get(&k).unwrap();
            if *kk > (1.0 - 1e-5) {
                return 1.0;
            }
        }

        return dp[&m][&m];
    }

    fn calculate_dp(i: i32, j: i32, dp: &HashMap<i32, HashMap<i32, f64>>) -> f64 {
        let dp0 = Self::dpget(dp, 0.max(i - 4), j);
        let dp1 = Self::dpget(dp, 0.max(i - 3), j - 1);
        let dp2 = Self::dpget(dp, 0.max(i - 2), 0.max(j - 2));
        let dp3 = Self::dpget(dp, i - 1, 0.max(j - 3));
        let sum = dp0 + dp1 + dp2 + dp3;
        return sum / 4.0;
    }

    fn dpget(dp: &HashMap<i32, HashMap<i32, f64>>, i: i32, j: i32) -> f64 {
        let r = dp.get(&i).unwrap().get(&j).unwrap();
        return *r;
    }
}

fn main() {
    let inputs = [50, 100];

    for n in inputs {
        let result = Solution::soup_servings(n);
        println!("{result}");
    }
}
