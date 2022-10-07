use std::collections::HashMap;

struct MyCalendarThree {
    vals: HashMap<i32, i32>,
    lazy: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            vals: HashMap::new(),
            lazy: HashMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.update(start, end - 1, 0, 1000000000, 1);
        return *self.vals.get(&1).unwrap_or(&0);
    }

    fn update(&mut self, start: i32, end: i32, left: i32, right: i32, idx: i32) {
        if (start > right) || (end < left) {
            return;
        }

        if (start <= left) && (right <= end) {
            if let Some(val) = self.vals.get_mut(&idx) {
                *val += 1;
            } else {
                self.vals.insert(idx, 1);
            }

            if let Some(val) = self.lazy.get_mut(&idx) {
                *val += 1;
            } else {
                self.lazy.insert(idx, 1);
            }
        } else {
            let mid = (left + right) / 2;
            let idx2 = idx * 2;
            let idx21 = idx2 + 1;

            self.update(start, end, left, mid, idx2);
            self.update(start, end, mid + 1, right, idx21);

            let val2 = self.vals.get(&idx2).unwrap_or(&0);
            let val21 = self.vals.get(&idx21).unwrap_or(&0);
            let newval = self.lazy.get(&idx).unwrap_or(&0) + val2.max(val21);
            self.vals.insert(idx, newval);
        }
    }
}

fn main() {
    let mut obj = MyCalendarThree::new();
    println!("{}", obj.book(10, 20));
    println!("{}", obj.book(50, 60));
    println!("{}", obj.book(10, 40));
    println!("{}", obj.book(5, 15));
    println!("{}", obj.book(5, 10));
    println!("{}", obj.book(25, 55));
}
