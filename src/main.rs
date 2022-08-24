struct MyQueue {
    current: Vec<i32>,
    buffer: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            current: vec![],
            buffer: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.buffer.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.fill();
        return self.current.pop().unwrap();
    }

    fn peek(&mut self) -> i32 {
        self.fill();
        let top = self.current.len() - 1;
        return self.current[top];
    }

    fn empty(&self) -> bool {
        let size = self.buffer.len() + self.current.len();
        return size < 1;
    }

    fn fill(&mut self) {
        let current = &mut self.current;
        let buffer = &mut self.buffer;

        if current.is_empty() {
            while let Some(n) = buffer.pop() {
                current.push(n);
            }
        }
    }
}

fn main() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    println!("{}", queue.peek());
    println!("{}", queue.pop());
    println!("{}", queue.empty());
}
