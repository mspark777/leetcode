struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        return StockSpanner { stack: vec![] };
    }

    fn next(&mut self, price: i32) -> i32 {
        let stack = &mut self.stack;
        let mut span = 1;

        while !stack.is_empty() {
            let top = stack.last().unwrap();
            let (p, s) = top;
            if *p <= price {
                span += s;
                stack.pop();
            } else {
                break;
            }
        }

        stack.push((price, span));
        return span;
    }
}

fn main() {
    let mut stock_spanner = StockSpanner::new();
    println!(
        "{} {} {} {} {} {} {}",
        stock_spanner.next(100),
        stock_spanner.next(80),
        stock_spanner.next(60),
        stock_spanner.next(70),
        stock_spanner.next(60),
        stock_spanner.next(75),
        stock_spanner.next(85)
    );
}
