struct Solution;

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, time: i32) -> i32 {
        clips.sort();

        let mut count = 0;
        let mut end = 0;
        let mut i = 0;

        while end < time {
            let mut max_reach = end;

            for j in i..clips.len() {
                if clips[j][0] > end {
                    break;
                }
                max_reach = max_reach.max(clips[j][1]);
            }
            if max_reach == end {
                return -1;
            }
            end = max_reach;
            count += 1;
            i += 1;
        }
        count
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}
