use std::cell::RefCell;

pub struct MyStack<T> {
    data: RefCell<Vec<T>>,
}

impl<T> MyStack<T> {
    pub fn new() -> MyStack<T> {
        MyStack { data: RefCell::new(Vec::new()) }
    }

    pub fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }

    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }
}