struct Solution {}

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let mut curr = 1i64;
        let mut k = k as i64;
        k -= 1;

        while k > 0 {
            let steps = Self::count_steps(n, curr, curr + 1);
            if steps <= k {
                curr += 1;
                k -= steps;
            } else {
                curr *= 10;
                k -= 1;
            }
        }

        return curr as i32;
    }

    fn count_steps(n: i64, prefix1: i64, prefix2: i64) -> i64 {
        let mut prefix1 = prefix1;
        let mut prefix2 = prefix2;
        let mut steps = 0i64;
        while prefix1 <= n {
            steps += (n + 1).min(prefix2) - prefix1;
            prefix1 *= 10;
            prefix2 *= 10;
        }

        return steps;
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input { n: 13, k: 2 },
        Input { n: 1, k: 1 },
        Input {
            n: 681692778,
            k: 351251360,
        },
    ];

    for input in inputs {
        let result = Solution::find_kth_number(input.n, input.k);
        println!("{:?}", result);
    }
}
