struct Solution {}

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut indexes = vec![0; 26];

        for (i, ch) in s.chars().enumerate() {
            let i = i as i32;
            let a = 'a' as usize;
            let code = ch as usize;
            let idx = code - a;

            let pos = indexes[idx];
            if pos == 0 {
                indexes[idx] -= i + 1;
                continue;
            }

            let diff = i + pos;
            if distance[idx] != diff {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    s: String,
    distance: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            s: "abaccb".to_string(),
            distance: [
                1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
            .to_vec(),
        },
        Input {
            s: "aa".to_string(),
            distance: [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
            .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::check_distances(input.s, input.distance);
        println!("{:?}", result);
    }
}
