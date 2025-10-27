struct Solution {}

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut current_energy = initial_energy;
        let mut current_experience = initial_experience;
        let mut result = 0;

        for (opponent_energy, opponent_experience) in
            energy.iter().cloned().zip(experience.iter().cloned())
        {
            if opponent_energy >= current_energy {
                let training = opponent_energy - current_energy + 1;
                result += training;
                current_energy += training;
            }

            if opponent_experience >= current_experience {
                let training = opponent_experience - current_experience + 1;
                result += training;
                current_experience += training;
            }

            current_energy -= opponent_energy;
            current_experience += opponent_experience;
        }

        result
    }
}

struct Input {
    initial_energy: i32,
    initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            initial_energy: 5,
            initial_experience: 3,
            energy: [1, 4, 3, 2].to_vec(),
            experience: [2, 6, 3, 1].to_vec(),
        },
        Input {
            initial_energy: 2,
            initial_experience: 4,
            energy: [1].to_vec(),
            experience: [3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_number_of_hours(
            input.initial_energy,
            input.initial_experience,
            input.energy,
            input.experience,
        );
        println!("{:?}", result);
    }
}
