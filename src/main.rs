struct Solution {}

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as usize;
        let mut low = 1;
        let mut high = cars * cars * (ranks[0] as usize);

        while low < high {
            let mid = (low + high) / 2;
            let mut cars_repaired = 0usize;

            for rank in ranks.iter().cloned() {
                let fmid = mid as f64;
                let rank = rank as f64;
                let repaired = (fmid / rank).sqrt();
                cars_repaired += repaired as usize;
            }

            if cars_repaired < cars {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        return low as i64;
    }
}

struct Input {
    ranks: Vec<i32>,
    cars: i32,
}

fn main() {
    let inputs = vec![
        Input {
            ranks: vec![4, 2, 3, 1],
            cars: 10,
        },
        Input {
            ranks: vec![5, 1, 8],
            cars: 6,
        },
    ];

    for input in inputs {
        let result = Solution::repair_cars(input.ranks, input.cars);
        println!("{result:?}");
    }
}
