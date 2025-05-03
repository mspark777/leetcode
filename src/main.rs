struct Solution {}

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let top = Self::get_count(tops[0], &tops, &bottoms);
        let bottom = Self::get_count(bottoms[0], &tops, &bottoms);
        let result = top.min(bottom);

        if result > tops.len() {
            return -1;
        }

        return result as i32;
    }

    fn get_count(target: i32, tops: &Vec<i32>, bottoms: &Vec<i32>) -> usize {
        let mut count1 = 0usize;
        let mut count2 = 0usize;
        for (top, bottom) in tops.iter().cloned().zip(bottoms.iter().cloned()) {
            if (top != target) && (bottom != target) {
                return tops.len() + 1;
            }

            if top == target {
                count1 += 1;
            }

            if bottom == target {
                count2 += 1;
            }
        }

        return tops.len() - count1.max(count2);
    }
}

struct Input {
    tops: Vec<i32>,
    bottoms: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            tops: vec![2, 1, 2, 4, 2, 2],
            bottoms: vec![5, 2, 6, 2, 3, 2],
        },
        Input {
            tops: vec![3, 5, 1, 2, 3],
            bottoms: vec![3, 6, 3, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::min_domino_rotations(input.tops, input.bottoms);
        println!("{result:?}");
    }
}
