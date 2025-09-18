#[derive(Debug)]
struct Deque<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(size: usize) -> Self {
        Deque {
            capacity: size,
            data: Vec::with_capacity(size),
        }
    }
    fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    // front is the right first one lifo
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.capacity {
            return Err("full capacity".to_string());
        }
        self.data.push(val);
        Ok(())
    }
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.capacity {
            return Err("full capacity".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }
    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut deque = Deque::new(4);
        let _r1 = deque.add_front("front1");
        let _r2 = deque.add_front("front2");
        let _r3 = deque.add_rear("rear3");
        let _r4 = deque.add_rear("rear4");
        if let Err(e) = deque.add_front("front3") {
            println!("add_front Error: {e}");
        }
        if let Some(data) = deque.remove_rear() {
            println!("remove_rear: {data}")
        } else {
            println!("empty deque")
        }
        println!("size: {}, empty: {}", deque.capacity, deque.is_empty());
        println!("content: {:?}", deque);
    }
}

pub fn is_palindrome(s: String) -> bool {
    let mut d: Vec<char> = Vec::new();
    let s: String = s
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_lowercase().nth(0).unwrap())
        .collect();
    for c in s.chars() {
        d.push(c);
    }
    let mut flag = true;
    while d.len() > 1 && flag {
        let head = d.pop().unwrap();
        let tail = d.remove(0);
        if head != tail {
            flag = false;
        }
    }
    flag
}
