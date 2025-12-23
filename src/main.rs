struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut u = 0;

        for m in moves.chars() {
            match m {
                'L' => l += 1,
                'R' => r += 1,
                _ => u += 1,
            }
        }

        l.max(r) + u - l.min(r)
    }
}

struct Input {
    moves: String,
}

fn main() {
    let inputs = [
        Input {
            moves: "L_RL__R".to_string(),
        },
        Input {
            moves: "_R__LL_".to_string(),
        },
        Input {
            moves: "_______".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::furthest_distance_from_origin(input.moves);
        println!("{:?}", result);
    }
}
