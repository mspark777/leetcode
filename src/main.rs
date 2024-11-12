struct Solution {}

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        return Self::solve(items.clone(), queries);
    }

    fn solve(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];

        items.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut max_beauty = 0;
        for item in items.iter_mut() {
            max_beauty = max_beauty.max(item[1]);
            item[1] = max_beauty;
        }

        for (i, &query) in queries.iter().enumerate() {
            result[i] = Self::binary_search(&items, query);
        }

        return result;
    }

    fn binary_search(items: &Vec<Vec<i32>>, target_price: i32) -> i32 {
        let mut left = 0usize;
        let mut right = items.len() - 1;
        let mut max_beauty = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let price = items[mid][0];
            if price > target_price {
                if mid < 1 {
                    break;
                } else {
                    right = mid - 1;
                }
            } else {
                let beauty = items[mid][1];
                max_beauty = max_beauty.max(beauty);
                left = mid + 1;
            }
        }

        return max_beauty;
    }
}

struct Input {
    items: Vec<Vec<i32>>,
    queries: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            items: vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            queries: vec![1, 2, 3, 4, 5, 6],
        },
        Input {
            items: vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
            queries: vec![1],
        },
        Input {
            items: vec![vec![10, 1000]],
            queries: vec![5],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_beauty(input.items, input.queries);
        println!("{result:?}");
    }
}
