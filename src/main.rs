struct Solution {}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut parents = (0..26usize).map(|i| i).collect::<Vec<_>>();

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            let u = (c1 as u8 - b'a') as usize;
            let v = (c2 as u8 - b'a') as usize;

            let pu = Self::find(&mut parents, u);
            let pv = Self::find(&mut parents, v);

            if pu < pv {
                parents[pv] = pu;
            } else {
                parents[pu] = pv;
            }
        }

        return base_str
            .chars()
            .map(|c| Self::convert(&mut parents, c))
            .collect();
    }

    fn find(parents: &mut Vec<usize>, i: usize) -> usize {
        if parents[i] != i {
            parents[i] = Self::find(parents, parents[i]);
        }

        return parents[i];
    }

    fn convert(parents: &mut Vec<usize>, c: char) -> char {
        let a = b'a';
        let u = ((c as u8) - a) as usize;
        let pu = Self::find(parents, u) as u8;
        return (pu + a) as char;
    }
}

struct Input {
    s1: &'static str,
    s2: &'static str,
    base_str: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            s1: "parker",
            s2: "morris",
            base_str: "parser",
        },
        Input {
            s1: "hello",
            s2: "world",
            base_str: "hold",
        },
        Input {
            s1: "leetcode",
            s2: "programs",
            base_str: "sourcecode",
        },
    ];

    for input in inputs {
        let result = Solution::smallest_equivalent_string(
            input.s1.to_string(),
            input.s2.to_string(),
            input.base_str.to_string(),
        );
        println!("{:?}", result);
    }
}
