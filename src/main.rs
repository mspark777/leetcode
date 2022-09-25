struct MyCircularQueue {
    queue: Vec<i32>,
    begin: usize,
    end: usize,
    size: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            queue: vec![0; k as usize],
            begin: 0,
            end: 0,
            size: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let end = self.end;
        self.queue[end] = value;
        self.end = self.next_index(end);
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.begin = self.next_index(self.begin);
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[self.begin]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        let end = self.end;
        let queue = &self.queue;
        let tail = if end == 0 { queue.len() - 1 } else { end - 1 };

        queue[tail]
    }

    fn is_empty(&self) -> bool {
        self.size < 1
    }

    fn is_full(&self) -> bool {
        self.size >= self.queue.len()
    }

    fn next_index(&self, cur: usize) -> usize {
        (cur + 1) % self.queue.len()
    }
}

fn main() {
    let mut queue = MyCircularQueue::new(3);
    println!("{}", queue.en_queue(1));
    println!("{}", queue.en_queue(2));
    println!("{}", queue.en_queue(3));
    println!("{}", queue.en_queue(4));
    println!("{}", queue.rear());
    println!("{}", queue.is_full());
    println!("{}", queue.de_queue());
    println!("{}", queue.en_queue(4));
    println!("{}", queue.rear());
}
