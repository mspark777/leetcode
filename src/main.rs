struct Solution {}

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let bulkey = Self::is_bulkey(length, width, height);
        let heavy = mass >= 100;

        match (bulkey, heavy) {
            (true, true) => "Both".to_string(),
            (true, false) => "Bulky".to_string(),
            (false, true) => "Heavy".to_string(),
            _ => "Neither".to_string(),
        }
    }

    fn is_bulkey(length: i32, width: i32, height: i32) -> bool {
        const MAX_DIMESTION: i32 = 10_000;
        if length >= MAX_DIMESTION {
            return true;
        } else if width >= MAX_DIMESTION {
            return true;
        } else if height >= MAX_DIMESTION {
            return true;
        }

        let volume = (length as i64) * (width as i64) * (height as i64);
        volume >= 1_000_000_000
    }
}

struct Input {
    length: i32,
    width: i32,
    height: i32,
    mass: i32,
}

fn main() {
    let inputs = [
        Input {
            length: 1000,
            width: 35,
            height: 700,
            mass: 300,
        },
        Input {
            length: 200,
            width: 50,
            height: 800,
            mass: 50,
        },
    ];

    for input in inputs {
        let result = Solution::categorize_box(input.length, input.width, input.height, input.mass);
        println!("{:?}", result);
    }
}
