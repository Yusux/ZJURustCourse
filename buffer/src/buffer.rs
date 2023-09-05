pub struct Buffer<T> {
    data: Vec<T>
}

impl<T: std::ops::Add<Output = T> + Clone> Buffer<T> {
    pub fn new() -> Buffer<T> {
        Buffer {
            data: Vec::new()
        }
    }

    pub fn sum(&self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let mut iter = self.data.iter();
        let mut sum = iter.next().unwrap().to_owned();
        for value in iter {
            sum = sum + value.clone();
        }
        Some(sum)
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if idx >= self.data.len() {
            return None;
        }
        Some(self.data.remove(idx))
    }
}