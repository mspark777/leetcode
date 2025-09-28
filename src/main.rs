struct Solution {}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut masks = vec![0u32; 10];
        let colors = rings.chars().step_by(2);
        let roads = rings.chars().skip(1).step_by(2);
        for (color, road) in colors.zip(roads) {
            let road_num = road.to_digit(10).unwrap() as usize;
            let color_mask = match color {
                'R' => 1u32,
                'G' => 2u32,
                'B' => 4u32,
                _ => unreachable!(),
            };
            masks[road_num] |= color_mask;
        }

        let mut result = 0;
        for mask in masks.iter().cloned() {
            if mask == 7 {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    rings: String,
}

fn main() {
    let inputs = [
        Input {
            rings: "B0B6G0R6R0R6G9".to_string(),
        },
        Input {
            rings: "B0R0G0R9R0B0G0".to_string(),
        },
        Input {
            rings: "G4".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_points(input.rings);
        println!("{:?}", result);
    }
}
