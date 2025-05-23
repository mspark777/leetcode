struct Solution {}

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let k = k as i64;
        let mut sum = 0i64;
        let mut count = 0i64;
        let mut positive_minimum = (1 << 30) as i64;
        let mut negative_maximum = -positive_minimum;

        for num in nums.iter().cloned() {
            let num = num as i64;
            let operated_node_value = num ^ k;
            sum += num;
            let net_change = operated_node_value - num;
            if net_change > 0 {
                positive_minimum = positive_minimum.min(net_change);
                sum += net_change;
                count += 1;
            } else {
                negative_maximum = negative_maximum.max(net_change);
            }
        }

        if (count & 1) == 0 {
            return sum;
        }

        let with_positive = sum - positive_minimum;
        let with_negative = sum + negative_maximum;
        return if with_positive > with_negative {
            with_positive
        } else {
            with_negative
        };
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 1],
            k: 3,
            edges: vec![vec![0, 1], vec![0, 2]],
        },
        Input {
            nums: vec![2, 3],
            k: 7,
            edges: vec![vec![0, 1]],
        },
        Input {
            nums: vec![7, 7, 7, 7, 7, 7],
            k: 3,
            edges: vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_value_sum(input.nums, input.k, input.edges);
        println!("{:?}", result);
    }
}
