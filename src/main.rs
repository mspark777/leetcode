struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let n = stones.len();
        let mut stones = stones;
        stones.sort();

        let mut i = 0;
        let mut low = n as i32;
        let high = std::cmp::max(
            stones[n - 1] - n as i32 + 2 - stones[1],
            stones[n - 2] - stones[0] - n as i32 + 2,
        );

        for j in 0..n {
            while stones[j] - stones[i] >= n as i32 {
                i += 1;
            }
            if ((j + 2 - i) == n) && (stones[j] + 2 - stones[i]) == (n as i32) {
                low = low.min(2);
            } else {
                low = low.min(n as i32 - (j - i + 1) as i32);
            }
        }
        vec![low, high]
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
