struct Solution {}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let total = 3 * (1 << (n - 1));
        if k > total {
            return "".to_string();
        }

        let mut next_smallest = std::collections::HashMap::<char, char>::with_capacity(3);
        next_smallest.insert('a', 'b');
        next_smallest.insert('b', 'a');
        next_smallest.insert('c', 'a');

        let mut next_greatest = std::collections::HashMap::<char, char>::with_capacity(3);
        next_greatest.insert('a', 'c');
        next_greatest.insert('b', 'c');
        next_greatest.insert('c', 'b');

        let mut k = k;
        let mut result = vec!['a'; n as usize];
        let start_a = 1;
        let start_b = start_a + (1 << (n - 1));
        let start_c = start_b + (1 << (n - 1));

        if k < start_b {
            k -= start_a;
        } else if k < start_c {
            result[0] = 'b';
            k -= start_b;
        } else {
            result[0] = 'c';
            k -= start_c;
        }

        for ch_idx in 1..n {
            let mid_point = 1 << (n - ch_idx - 1);
            if k < mid_point {
                result[ch_idx as usize] = next_smallest[&result[ch_idx as usize - 1]];
            } else {
                result[ch_idx as usize] = next_greatest[&result[ch_idx as usize - 1]];
                k -= mid_point;
            }
        }

        return result.iter().collect();
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input { n: 1, k: 3 },
        Input { n: 1, k: 4 },
        Input { n: 3, k: 9 },
    ];

    for input in inputs {
        let result = Solution::get_happy_string(input.n, input.k);
        println!("{result:?}");
    }
}
