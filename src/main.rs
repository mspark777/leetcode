struct Solution {}

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        match k <= (num_ones + num_zeros) {
            true => num_ones.min(k),
            false => num_ones * 2 + num_zeros - k,
        }
    }
}

struct Input {
    num_ones: i32,
    num_zeros: i32,
    num_neg_ones: i32,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            num_ones: 3,
            num_zeros: 2,
            num_neg_ones: 0,
            k: 2,
        },
        Input {
            num_ones: 3,
            num_zeros: 2,
            num_neg_ones: 0,
            k: 4,
        },
    ];

    for input in inputs {
        let result = Solution::k_items_with_maximum_sum(
            input.num_ones,
            input.num_zeros,
            input.num_neg_ones,
            input.k,
        );
        println!("{:?}", result);
    }
}
