struct MyStack {
    queue: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack { queue: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let queue = &mut self.queue;
        queue.push(x);

        let size = queue.len();
        for _ in 1..size {
            let top = queue.remove(0);
            queue.push(top);
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.remove(0)
    }

    fn top(&self) -> i32 {
        self.queue[0]
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let mut my_stack = MyStack::new();
    my_stack.push(1);
    my_stack.push(2);

    println!("{}", my_stack.top());
    println!("{}", my_stack.pop());
    println!("{}", my_stack.empty());
}
