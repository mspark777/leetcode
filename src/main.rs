struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut result = 0;

        while left < right {
            let lheight = height[left];
            let rheight = height[right];
            if lheight < rheight {
                left += 1;
                if lheight >= left_max {
                    left_max = lheight;
                } else {
                    result += left_max - lheight;
                }
            } else {
                right -= 1;
                if rheight >= right_max {
                    right_max = rheight;
                } else {
                    result += right_max - rheight;
                }
            }
        }

        result
    }
}

struct Input {
    height: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            height: vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        },
        Input {
            height: vec![4, 2, 0, 3, 2, 5],
        },
    ];

    for Input { height } in inputs.into_iter() {
        let result = Solution::trap(height);
        println!("{result:?}");
    }
}
