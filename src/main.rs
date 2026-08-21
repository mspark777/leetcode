struct Solution;

impl Solution {
    pub fn stone_game(_piles: Vec<i32>) -> bool {
        true
    }
}

struct Input {
    piles: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        piles: [5, 3, 4, 5].to_vec(),
    }];

    for input in inputs {
        let result = Solution::stone_game(input.piles);
        println!("{:?}", result);
    }
}
