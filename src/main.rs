struct Solution {}

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 0;
        let mut right = *candies.iter().max().unwrap();

        while left < right {
            let middle = (left + right + 1) / 2;
            if Self::can_allocate_candies(&candies, k, middle) {
                left = middle;
            } else {
                right = middle - 1;
            }
        }

        return left;
    }

    fn can_allocate_candies(candies: &Vec<i32>, k: i64, num_of_candies: i32) -> bool {
        let mut max_num_of_children = 0i64;

        for pile in candies.iter().cloned() {
            max_num_of_children += (pile / num_of_candies) as i64;
        }

        return max_num_of_children >= k;
    }
}

struct Input {
    candies: Vec<i32>,
    k: i64,
}

fn main() {
    let inputs = vec![
        Input {
            candies: vec![5, 8, 6],
            k: 3,
        },
        Input {
            candies: vec![2, 5],
            k: 11,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_candies(input.candies, input.k);
        println!("{result:?}");
    }
}
