struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let land = land_start_time
            .iter()
            .copied()
            .zip(land_duration.iter().copied())
            .map(|(s, d)| s + d)
            .min()
            .unwrap();

        let water = water_start_time
            .iter()
            .copied()
            .zip(water_duration.iter().copied())
            .map(|(s, d)| s + d)
            .min()
            .unwrap();

        let water_land = land_start_time
            .iter()
            .copied()
            .zip(land_duration.iter().copied())
            .map(|(s, d)| s.max(water) + d)
            .min()
            .unwrap();

        let land_water = water_start_time
            .iter()
            .copied()
            .zip(water_duration.iter().copied())
            .map(|(s, d)| s.max(land) + d)
            .min()
            .unwrap();

        water_land.min(land_water)
    }
}

struct Input {
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        land_start_time: [2, 8].to_vec(),
        land_duration: [4, 1].to_vec(),
        water_start_time: [6].to_vec(),
        water_duration: [3].to_vec(),
    }];

    for input in inputs {
        let result = Solution::earliest_finish_time(
            input.land_start_time,
            input.land_duration,
            input.water_start_time,
            input.water_duration,
        );
        println!("{:?}", result);
    }
}
