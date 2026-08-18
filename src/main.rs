struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let end = n + 1;
        let mut rslt = 0usize;
        let mut start = end;

        for (i, seat) in seats.into_iter().enumerate() {
            if seat == 1 {
                if start == end {
                    rslt = i;
                } else {
                    rslt = rslt.max((i - start) / 2);
                }
                start = i;
            }
        }

        rslt.max(n - 1 - start) as i32
    }
}

struct Input {
    seats: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        seats: [1, 0, 0, 0, 1, 0, 1].to_vec(),
    }];

    for input in inputs {
        let result = Solution::max_dist_to_closest(input.seats);
        println!("{:?}", result);
    }
}
