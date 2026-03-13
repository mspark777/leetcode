struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let steps1 = (x - z).abs();
        let steps2 = (y - z).abs();

        if steps1 < steps2 {
            1
        } else if steps1 > steps2 {
            2
        } else {
            0
        }
    }
}

struct Input {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let inputs = [
        Input { x: 2, y: 7, z: 4 },
        Input { x: 2, y: 5, z: 6 },
        Input { x: 1, y: 5, z: 3 },
    ];

    for input in inputs {
        let result = Solution::find_closest(input.x, input.y, input.z);
        println!("{:?}", result);
    }
}
