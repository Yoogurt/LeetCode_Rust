struct MyQueue {
    stack_1: Vec<i32>,
    stack_2: Vec<i32>,
}

impl Default for MyQueue {
    fn default() -> Self {
        MyQueue {
            stack_1: vec![],
            stack_2: vec![],
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue::default()
    }

    fn push(&mut self, x: i32) {
        self.stack_1.push(x)
    }

    fn fill_stack(&mut self) {
        while let Some(x) = self.stack_1.pop() {
            self.stack_2.push(x)
        }
    }

    fn pop(&mut self) -> i32 {
        if !self.stack_2.is_empty() {
            return self.stack_2.pop().unwrap();
        }

        self.fill_stack();
        return self.stack_2.pop().unwrap();
    }

    fn peek(&mut self) -> i32 {
        if !self.stack_2.is_empty() {
            return *self.stack_2.last().unwrap();
        }

        self.fill_stack();
        return *self.stack_2.last().unwrap();
    }

    fn empty(&self) -> bool {
        self.stack_1.is_empty() && self.stack_2.is_empty()
    }
}

#[test]
fn test_code_232() {
    let mut queue = MyQueue::default();

    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.peek(), 2);
    assert_eq!(queue.pop(), 2);
    assert_eq!(queue.empty(), true);
}