struct Solution {}
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut p = p;
        let mut q = q;
        while ((p % 2) + (q % 2)) == 0 {
            p /= 2;
            q /= 2;
        }

        return (q % 2) - (p % 2) + 1;
    }
}

struct Input {
    p: i32,
    q: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { p: 2, q: 1 }, Input { p: 3, q: 1 }];

    for input in inputs {
        let p = input.p;
        let q = input.q;
        let result = Solution::mirror_reflection(p, q);
        println!("{:?}", result);
    }
}
