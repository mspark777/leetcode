mod rs;

struct Input {
    heights: Vec<i32>,
    bricks: i32,
    leadders: i32,
}

fn main() {
    let inputs = [
        Input {
            heights: vec![4, 2, 7, 6, 9, 14, 12],
            bricks: 5,
            leadders: 1,
        },
        Input {
            heights: vec![4, 12, 2, 7, 3, 18, 20, 3, 19],
            bricks: 10,
            leadders: 2,
        },
        Input {
            heights: vec![14, 3, 19, 3],
            bricks: 17,
            leadders: 0,
        },
        Input {
            heights: vec![1, 5, 1, 2, 3, 4, 10000],
            bricks: 4,
            leadders: 1,
        },
    ];

    for input in inputs {
        let result = rs::furthest_building::FurthestBuilding::furthest_building(
            input.heights,
            input.bricks,
            input.leadders,
        );
        println!("{result:?}");
    }
}
