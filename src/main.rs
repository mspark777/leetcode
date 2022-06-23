mod solution;

struct Input {
    courses: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            courses: vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200],
            ],
        },
        Input {
            courses: vec![vec![1, 2]],
        },
        Input {
            courses: vec![vec![3, 2], vec![4, 3]],
        },
        Input {
            courses: vec![
                vec![5, 15],
                vec![3, 19],
                vec![6, 7],
                vec![2, 10],
                vec![5, 16],
                vec![8, 14],
                vec![10, 11],
                vec![2, 19],
            ],
        },
    ];

    for input in inputs {
        let result = solution::Solution::schedule_course(input.courses);
        println!("{result:?}");
    }
}
