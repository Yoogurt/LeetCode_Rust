use std::collections::VecDeque;

struct MyStack {
    left_stack: VecDeque<i32>,
    right_stack: VecDeque<i32>,
    use_left_stack: bool,
}

impl Default for MyStack {
    fn default() -> Self {
        MyStack {
            left_stack: VecDeque::new(),
            right_stack: VecDeque::new(),
            use_left_stack: true,
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack::default()
    }

    fn push(&mut self, x: i32) {
        if self.use_left_stack {
            self.left_stack.push_back(x)
        } else {
            self.right_stack.push_back(x);
        }
    }

    fn fill_backup_queue(&mut self) {
        let stack_to_fill: &mut VecDeque<i32>;
        let stack_to_empty: &mut VecDeque<i32>;

        if self.use_left_stack {
            stack_to_fill = &mut self.right_stack;
            stack_to_empty = &mut self.left_stack;
        } else {
            stack_to_fill = &mut self.left_stack;
            stack_to_empty = &mut self.right_stack;
        };

        while stack_to_empty.len() > 1 {
            stack_to_fill.push_back(stack_to_empty.pop_front().unwrap())
        }
    }

    fn pop(&mut self) -> i32 {
        if self.use_left_stack {
            if self.left_stack.is_empty() {
                self.use_left_stack = false;
            }
        } else if self.right_stack.is_empty() {
            self.use_left_stack = true;
        }

        self.fill_backup_queue();

        if self.use_left_stack {
            self.left_stack.pop_front().unwrap()
        } else {
            self.right_stack.pop_front().unwrap()
        }
    }

    fn top(&mut self) -> i32 {
        if self.use_left_stack {
            if self.left_stack.is_empty() {
                self.use_left_stack = false;
            }
        } else if self.right_stack.is_empty() {
            self.use_left_stack = true;
        }

        self.fill_backup_queue();

        if self.use_left_stack {
            *self.left_stack.front().unwrap()
        } else {
            *self.right_stack.front().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.left_stack.is_empty() && self.right_stack.is_empty()
    }
}

#[test]
fn test_code_225() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);

    stack.push(3);
    stack.push(4);
    stack.push(5);

    assert_eq!(stack.pop(), 5);
    assert_eq!(stack.top(), 4);
    assert_eq!(stack.pop(), 4);
    assert_eq!(stack.pop(), 3);
    assert_eq!(stack.pop(), 1);

    assert_eq!(stack.empty(), true);
}