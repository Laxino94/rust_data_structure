#[derive(Debug)]
pub struct Stacks<T> {
    top: usize,   // stack top index
    data: Vec<T>, // stack data
}

impl<T> Stacks<T> {
    // return a new init empty stack
    pub fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, value: T) {
        self.data.push(value); // data will be at the end of the vector
        self.top += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1; // adjust top
            self.data.pop() // pop the last element
        }
    }
    pub fn peak(&self) -> Option<&T> {
        if self.top == 0 {
            None
        } else {
            self.data.get(self.top - 1) // ref to the top element not ownership
        }
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn size(&self) -> usize {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut stack = Stacks::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        println!("top: {:?}", stack.peak().unwrap());
        println!("size: {}", stack.size());
        println!("pop: {:?}, size: {:?}", stack.pop(), stack.size());
        println!("is_empty: {}", stack.is_empty());
    }
}
