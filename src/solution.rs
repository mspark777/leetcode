pub struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue: Vec<Vec<i32>> = people
            .iter()
            .map(|person| vec![person[0], person[1]])
            .collect();

        queue.sort_unstable_by_key(|person| (-person[0], person[1]));

        let mut result = Vec::<Vec<i32>>::with_capacity(people.len());
        for person in queue {
            result.insert(person[1] as usize, person.clone());
        }

        result
    }
}
