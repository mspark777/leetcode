struct Solution {}

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        return Self::solve(&mut robot.clone(), &mut factory.clone());
    }

    fn solve(robots: &mut Vec<i32>, factories: &mut Vec<Vec<i32>>) -> i64 {
        robots.sort_unstable();
        factories.sort_unstable_by_key(|v| v[0]);

        let mut factory_positions = Vec::<i32>::new();
        for factory in factories.iter() {
            for _ in 0..factory[1] {
                factory_positions.push(factory[0]);
            }
        }

        let robot_count = robots.len();
        let last_robot_count = robot_count - 1;
        let factory_count = factory_positions.len();
        let mut next = vec![0i64; factory_count + 1];
        let mut current = vec![0i64; next.len()];

        for i in (0..robot_count).rev() {
            if i != last_robot_count {
                next[factory_count] = 1e12 as i64;
            }

            current[factory_count] = 1e12 as i64;
            for j in (0..factory_count).rev() {
                let a = (robots[i] - factory_positions[j]).abs() as i64;
                let assign = a + next[j + 1];
                let skip = current[j + 1];
                current[j] = assign.min(skip);
            }

            next = current.clone();
        }

        return current[0];
    }
}

struct Input {
    robot: Vec<i32>,
    factory: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            robot: vec![0, 4, 6],
            factory: vec![vec![2, 2], vec![6, 2]],
        },
        Input {
            robot: vec![1, -1],
            factory: vec![vec![-2, 1], vec![2, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_total_distance(input.robot, input.factory);
        println!("{result:?}");
    }
}
