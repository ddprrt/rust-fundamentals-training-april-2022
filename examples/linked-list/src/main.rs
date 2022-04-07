pub struct SimpleLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T: Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }
    
    pub fn push(&mut self, _element: T) {
        let top = Box::new(Node {
            value: _element,
            next: self.head.take(),
        });
        self.head = Some(top);
        self.len = self.len + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(x) => {
                self.head = x.next;
                self.len = self.len - 1;
                Some(x.value)
            }
            None => None,
        }
    }
}

pub struct PeekingIterator<'a, S: Clone> {
    ptr: &'a Option<Box<Node<S>>>
}

impl<'a, S: Clone> PeekingIterator<'a, S> {
    fn new(list: &'a SimpleLinkedList<S>) -> Self {
        let ptr = &list.head;
        Self { ptr }
    }
}

impl<'a, S: Clone> Iterator for PeekingIterator<'a, S> {
    type Item = &'a S;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            Some(val) => {
                let x = &val.value;
                self.ptr = &val.next;
                Some(x)
            }
            None => None
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a SimpleLinkedList<T> {
    type Item = &'a T;

    type IntoIter = PeekingIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PeekingIterator::new(self)
    }
}



impl<T: Clone> Iterator for SimpleLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

fn main() {
    let mut list = SimpleLinkedList::new();
    list.push(3);
    list.push(4);

    for i in &list {
        println!("{}", i);
    }

    for i in list {
        println!("{}", i);
    }
}

