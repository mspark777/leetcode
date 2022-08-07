struct Solution {}
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: u64 = 1000000007;
        let mut a = 1u64;
        let mut e = 1u64;
        let mut i = 1u64;
        let mut o = 1u64;
        let mut u = 1u64;

        for _ in 1..n {
            let nexta = e + i + u;
            let nexte = a + i;
            let nexti = e + o;
            let nexto = i;
            let nextu = i + o;

            a = nexta % MOD;
            e = nexte % MOD;
            i = nexti % MOD;
            o = nexto % MOD;
            u = nextu % MOD;
        }

        ((a + e + i + o + u) % MOD) as i32
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { n: 1 },
        Input { n: 2 },
        Input { n: 5 },
        Input { n: 144 },
    ];

    for input in inputs {
        let n = input.n;
        let result = Solution::count_vowel_permutation(n);
        println!("{:?}", result);
    }
}
