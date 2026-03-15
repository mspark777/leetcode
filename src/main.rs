struct Solution;

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        const A: usize = 'a' as usize;
        let mut frequencies = [0i32; 26];
        let mut distincts = 0;

        for ch in s.chars() {
            let code = ch as usize;
            let idx = code - A;

            frequencies[idx] += 1;

            if frequencies[idx] == 1 {
                distincts += 1;
            }
        }

        let k = k as usize;
        if k >= distincts {
            return 0;
        }

        frequencies.sort();
        frequencies
            .iter()
            .copied()
            .filter(|&n| n > 0)
            .take(distincts - k)
            .sum()
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "abc".to_string(),
            k: 2,
        },
        Input {
            s: "aabb".to_string(),
            k: 2,
        },
        Input {
            s: "yyyzz".to_string(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::min_deletion(input.s, input.k);
        println!("{:?}", result);
    }
}
