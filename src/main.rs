struct Solution {}
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        const LEFT: u8 = b'L';
        const RIGHT: u8 = b'R';
        const STAND: u8 = b'.';

        let bytes = dominoes.as_bytes();
        let length = dominoes.len();

        let mut force = 0;
        let mut forces = vec![0; length];
        for i in 0..length {
            let ch = bytes[i];
            if ch == LEFT {
                force = 0;
            } else if ch == RIGHT {
                force = length as i32;
            } else {
                force = force.max(1) - 1;
            }

            forces[i] += force
        }

        force = 0;
        for i in (0..length).rev() {
            let ch = bytes[i];
            if ch == LEFT {
                force = length as i32;
            } else if ch == RIGHT {
                force = 0;
            } else {
                force = force.max(1) - 1;
            }

            forces[i] -= force
        }

        let mut result = vec![0u8; length];
        for (i, force) in forces.into_iter().enumerate() {
            if force < 0 {
                result[i] = LEFT;
            } else if force > 0 {
                result[i] = RIGHT;
            } else {
                result[i] = STAND;
            }
        }

        String::from_utf8_lossy(result.as_slice()).to_string()
    }
}

fn main() {
    let inputs = vec!["RR.L", ".L.R...LR..L.."];

    for input in inputs.into_iter() {
        let dominoes = input.to_string();
        let result = Solution::push_dominoes(dominoes);
        println!("{result}");
    }
}
