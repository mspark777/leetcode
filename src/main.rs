struct Solution {}

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.chars().collect::<Vec<char>>();
        let mut result = vec![0; boxes.len()];

        let mut balls_to_left = 0;
        let mut moves_to_left = 0;
        let mut balls_to_right = 0;
        let mut moves_to_right = 0;

        for i in 0..boxes.len() {
            result[i] += moves_to_left;
            balls_to_left += (boxes[i] as i32) - ('0' as i32);
            moves_to_left += balls_to_left;

            let j = boxes.len() - 1 - i;
            result[j] += moves_to_right;
            balls_to_right += (boxes[j] as i32) - ('0' as i32);
            moves_to_right += balls_to_right;
        }

        return result;
    }
}

struct Input {
    boxes: &'static str,
}

fn main() {
    let inputs = vec![Input { boxes: "110" }, Input { boxes: "001011" }];

    for input in inputs {
        let result = Solution::min_operations(input.boxes.to_string());
        println!("{result:?}");
    }
}
