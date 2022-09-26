struct Solution {}
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parents: Vec<usize> = (0..=26).collect();

        for equation in equations.iter() {
            let bytes = equation.as_bytes();
            if bytes[1] == b'=' {
                let a = (bytes[0] - b'a') as usize;
                let b = (bytes[3] - b'a') as usize;
                Self::union(&mut parents, a, b);
            }
        }

        for equation in equations.iter() {
            let bytes = equation.as_bytes();
            if bytes[1] == b'!' {
                let a = (bytes[0] - b'a') as usize;
                let b = (bytes[3] - b'a') as usize;

                let parenta = Self::find(&mut parents, a);
                let parentb = Self::find(&mut parents, b);
                if parenta == parentb {
                    return false;
                }
            }
        }

        return true;
    }

    fn find(parents: &mut Vec<usize>, code: usize) -> usize {
        let mut parent = parents[code];
        if parent == code {
            return parent;
        }

        parent = Self::find(parents, parent);
        parents[code] = parent;
        return parent;
    }

    fn union(parents: &mut Vec<usize>, a: usize, b: usize) {
        let parenta = Self::find(parents, a);
        let parentb = Self::find(parents, b);
        parents[parentb] = parenta;
    }
}

fn main() {
    let inputs = vec![
        vec!["a==b", "b!=a"],
        vec!["b==a", "a==b"],
        vec!["c==c", "b==d", "x!=z"],
    ];

    for input in inputs.into_iter() {
        let equations = input.iter().map(|s| s.to_string()).collect();
        let result = Solution::equations_possible(equations);
        println!("{result}");
    }
}
