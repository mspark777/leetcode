struct Solution {}

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .cloned()
            .fold((-100000, 100000), |(result, distance), cur| {
                let d = cur.abs();
                if d < distance {
                    (cur, d)
                } else if d == distance {
                    (result.max(cur), d)
                } else {
                    (result, distance)
                }
            })
            .0
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [-4, -2, 1, 4, 8].to_vec(),
        },
        Input {
            nums: [2, -1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_closest_number(input.nums);
        println!("{:?}", result);
    }
}
