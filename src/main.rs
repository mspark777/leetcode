use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        const NOT_FOUND: i32 = -1;
        let mut queue = BinaryHeap::<i32>::with_capacity(stations.len());
        let mut result = 0;
        let mut prev = 0;
        let mut tank = start_fuel;

        for station in stations.iter() {
            let position = station[0];
            tank -= position - prev;
            result += Self::update_tank(&mut queue, &mut tank);
            if tank < 0 {
                return NOT_FOUND;
            }

            let fuel = station[1];
            queue.push(fuel);
            prev = position;
        }

        tank -= target - prev;
        result += Self::update_tank(&mut queue, &mut tank);
        if tank < 0 {
            return NOT_FOUND;
        }
        result
    }

    fn update_tank(queue: &mut BinaryHeap<i32>, tank: &mut i32) -> i32 {
        let mut result = 0;
        let mut temptank = *tank;
        while temptank < 0 {
            if let Some(fuel) = queue.pop() {
                temptank += fuel;
                result += 1;
            } else {
                break;
            }
        }

        *tank = temptank;
        result
    }
}

struct Input {
    target: i32,
    start_fuel: i32,
    stations: Vec<Vec<i32>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            target: 1,
            start_fuel: 1,
            stations: vec![],
        },
        Input {
            target: 100,
            start_fuel: 1,
            stations: vec![vec![10, 100]],
        },
        Input {
            target: 100,
            start_fuel: 10,
            stations: vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]],
        },
        Input {
            target: 100,
            start_fuel: 50,
            stations: vec![vec![50, 50]],
        },
        Input {
            target: 100,
            start_fuel: 50,
            stations: vec![vec![25, 50], vec![50, 25]],
        },
    ];

    for input in inputs {
        let target = input.target;
        let start_fuel = input.start_fuel;
        let stations = input.stations;
        let result = Solution::min_refuel_stops(target, start_fuel, stations);
        println!("{:?}", result);
    }
}
