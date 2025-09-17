#[derive(Debug)]
struct Stack<T> {
    top: usize,   // stack top index
    data: Vec<T>, // stack data
}

impl<T> Stack<T> {
    // return a new init empty stack
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.data.push(value); // data will be at the end of the vector
        self.top += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1; // adjust top
            self.data.pop() // pop the last element
        }
    }
    fn pick(&self) -> Option<&T> {
        if self.top == 0 {
            None
        } else {
            self.data.get(self.top - 1) // ref to the top element not ownership
        }
    }
    fn is_empty(&self) -> bool {
        self.top == 0
    }
    fn size(&self) -> usize {
        self.top
    }
}
