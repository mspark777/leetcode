struct ProductOfNumbers {
    prefix_product: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        return Self {
            prefix_product: vec![1],
        };
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_product = vec![1];
        } else {
            let last = self.prefix_product.last().unwrap();
            self.prefix_product.push(num * last);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let size = self.prefix_product.len();
        let k = k as usize;
        if k >= size {
            return 0;
        }

        let last = size - 1;
        return self.prefix_product[last] / self.prefix_product[last - k];
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![2, 11, 10, 1, 3],
            k: 10,
        },
        Input {
            nums: vec![1, 1, 2, 4, 9],
            k: 20,
        },
        Input {
            nums: vec![
                1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999,
            ],
            k: 1000000000,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{result:?}");
    }
}
