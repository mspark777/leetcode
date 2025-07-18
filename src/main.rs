struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n3 = nums.len();
        let n = n3 / 3;
        let mut part1 = vec![0i64; n + 1];
        let mut sum = 0i64;
        let mut max_heap = std::collections::BinaryHeap::<i64>::with_capacity(part1.len());
        for num in nums.iter().take(n).cloned() {
            let num = num as i64;
            sum += num;
            max_heap.push(num);
        }
        part1[0] = sum;
        for (i, num) in nums.iter().skip(n).take(n).cloned().enumerate() {
            let num = num as i64;
            sum += num;
            max_heap.push(num);
            sum -= max_heap.pop().unwrap();
            part1[i + 1] = sum;
        }
        let mut part2 = 0i64;
        let mut min_heap =
            std::collections::BinaryHeap::<std::cmp::Reverse<i64>>::with_capacity(part1.len());
        for num in nums.iter().skip(n * 2).take(n).rev().cloned() {
            let num = num as i64;
            part2 += num;
            min_heap.push(std::cmp::Reverse(num));
        }

        let mut result = part1[n] - part2;
        for (i, num) in nums.iter().skip(n).take(n).cloned().enumerate().rev() {
            let num = num as i64;
            part2 += num;
            min_heap.push(std::cmp::Reverse(num));
            if let Some(std::cmp::Reverse(val)) = min_heap.pop() {
                part2 -= val;
            }
            result = result.min(part1[i] - part2);
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: [3, 1, 2].to_vec(),
        },
        Input {
            nums: [7, 9, 5, 8, 1, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_difference(input.nums);
        println!("{:?}", result);
    }
}
