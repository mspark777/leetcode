/*
leetcode
 */

struct Solution {}
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        let mut cur = k;

        for &i in num.iter().rev() {
            cur += i;
            result.push(cur % 10);
            cur /= 10;
        }

        while cur > 0 {
            result.push(cur % 10);
            cur /= 10;
        }

        result.reverse();
        return result;
    }
}

fn main() {
    let inputs: Vec<(Vec<i32>, i32)> = vec![
        (vec![1, 2, 0, 0], 34),
        (vec![2, 7, 4], 181),
        (vec![2, 1, 5], 806),
        (
            vec![1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3],
            516,
        ),
        (vec![0], 10000),
    ];

    for (num, k) in inputs {
        let result = Solution::add_to_array_form(num, k);
        println!("{result:?}");
    }
}
