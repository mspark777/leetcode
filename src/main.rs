struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        for asteroid in asteroids {
            if asteroid > 0 {
                result.push(asteroid);
                continue;
            }

            while let Some(&val) = result.last()
                && val > 0
                && val < (-asteroid)
            {
                result.pop();
            }

            if let Some(&val) = result.last()
                && val > 0
            {
                if val == (-asteroid) {
                    result.pop();
                }

                continue;
            }

            result.push(asteroid);
        }

        result
    }
}

struct Input {
    asteroids: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            asteroids: [5, 10, -5].to_vec(),
        },
        Input {
            asteroids: [8, -8].to_vec(),
        },
        Input {
            asteroids: [10, 2, -5].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::asteroid_collision(input.asteroids);
        println!("{:?}", result);
    }
}
