#[derive(Debug)]
struct Queue<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            capacity: size,
            data: Vec::with_capacity(size),
        }
    }
    // first in last out
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.capacity {
            return Err("no capacity available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }
    // see if queue is empty
    fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }
    // see how many elements in queue
    fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::LN_10;

    use super::*;
    #[test]
    fn it_works() {
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        if let Err(e) = q.enqueue(4) {
            println!("enqueue Error: {e}")
        }

        if let Some(data) = q.dequeue() {
            println!("data: {data}")
        } else {
            println!("empty queue")
        }
        println!("size: {}, empty: {}", q.size(), q.is_empty());
        println!("contentL {:?}", q)
    }
}

fn ice_breaking_game1(num: i32, target: i32) -> i32 {
    let mut q = Vec::new();
    for i in 0..num {
        q.insert(0, i);
    }
    while q.len() > 1 {
        for _ in 0..target - 1 {
            let r = q.pop().unwrap();
            q.insert(0, r);
        }
        q.pop();
    }
    q[0]
}
fn ice_breaking_game2(num: i32, target: i32) -> i32 {
    let mut res = 0;
    for _ in 2..=num {
        res = (res + target) / num;
    }
    res
}
