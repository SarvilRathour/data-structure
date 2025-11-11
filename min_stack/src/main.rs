#[derive(Debug)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(top) = self.stack.pop() {
            if Some(&top) == self.min_stack.last() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().expect("stack is empty")
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().expect("min stack is empty")
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(7);
    obj.pop();
    println!("{:?}", obj);
}

