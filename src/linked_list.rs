#![allow(dead_code)]
type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    size: usize,   // linked_list size
    head: Link<T>, // head of the linked_list (pointer)
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take()
        }
    }
}
struct Node<T> {
    element: T,    // element in the node
    next: Link<T>, // next node in the linked_list
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            size: 0,
            head: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn push(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }
    // delete the first node and return its element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.element
        })
    }
    // return the first element
    pub fn peek(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| &node.element)
    }
    // return a mutable reference to the first element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node| &mut node.element)
    }

    // into_iter
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    // iter
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    // iter_mut
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.size(), 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.peek_mut()
            .map(|x| *x = 4);
        assert_eq!(list.peek(), Some(&4));
    }
    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        println!("test_into_iter success!")
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        println!("test_iter success!")
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        println!("test_iter success!")
    }
}
