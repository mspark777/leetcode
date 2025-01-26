struct Solution {}

impl Solution {
    pub fn maximum_invitations(favorites: Vec<i32>) -> i32 {
        let n = favorites.len();
        let mut indegrees = vec![0; n];

        for f in favorites.iter().cloned() {
            let person = f as usize;
            indegrees[person] += 1;
        }

        let mut queue = std::collections::VecDeque::<usize>::with_capacity(n);
        for (person, indegree) in indegrees.iter().cloned().enumerate() {
            if indegree == 0 {
                queue.push_back(person);
            }
        }

        let mut depths = vec![1; n];
        while let Some(current) = queue.pop_front() {
            let next = favorites[current] as usize;
            depths[next] = depths[next].max(depths[current] + 1);
            if indegrees[next] == 1 {
                queue.push_back(next);
            }

            indegrees[next] -= 1;
        }

        let mut longest_cycle = 0;
        let mut two_cycle_invitations = 0;
        for person in 0..n {
            if indegrees[person] == 0 {
                continue;
            }

            let mut cycle_length = 0;
            let mut current = person;
            while indegrees[current] != 0 {
                indegrees[current] = 0;
                cycle_length += 1;
                current = favorites[current] as usize;
            }

            if cycle_length == 2 {
                two_cycle_invitations += depths[person] + depths[favorites[person] as usize];
            } else {
                longest_cycle = longest_cycle.max(cycle_length);
            }
        }

        return longest_cycle.max(two_cycle_invitations);
    }
}

struct Input {
    favorite: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            favorite: vec![2, 2, 1, 2],
        },
        Input {
            favorite: vec![1, 2, 0],
        },
        Input {
            favorite: vec![3, 0, 1, 4, 1],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_invitations(input.favorite);
        println!("{result:?}");
    }
}
