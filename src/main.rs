struct Solution {}

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        (main_tank + additional_tank.min((main_tank - 1) / 4)) * 10
    }
}

struct Input {
    main_tank: i32,
    adadditional_tank: i32,
}

fn main() {
    let inputs = [
        Input {
            main_tank: 5,
            adadditional_tank: 10,
        },
        Input {
            main_tank: 1,
            adadditional_tank: 2,
        },
    ];

    for input in inputs {
        let result = Solution::distance_traveled(input.main_tank, input.adadditional_tank);
        println!("{:?}", result);
    }
}
