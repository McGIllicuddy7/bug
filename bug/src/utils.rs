#[derive(Clone, Debug, Copy)]
pub struct Stream<'a, T: Clone + std::fmt::Debug> {
    pub values: &'a [T],
}
impl<'a, T: Clone + std::fmt::Debug> Stream<'a, T> {
    pub fn new(vs: &'a [T]) -> Self {
        return Self { values: vs };
    }
    pub fn peek(&self) -> Option<T> {
        let mut prev = self.clone();
        prev.next()
    }
    pub fn collect_until(&mut self, done: &mut dyn FnMut(&[T]) -> bool) -> Self {
        let mut tmp = &self.values[0..0];
        let mut i = 0;
        while !done(tmp) {
            if i >= self.values.len() {
                break;
            }
            tmp = &self.values[0..i];
            i += 1;
        }
        self.values = &self.values[i..];
        Self { values: tmp.into() }
    }
    pub fn collect(&self) -> Vec<T> {
        self.values.to_owned()
    }
}
impl<'a, T: Clone + std::fmt::Debug> Iterator for Stream<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.values.len() > 0 {
            let p = self.values[0].clone();
            self.values = &self.values[1..];
            Some(p)
        } else {
            None
        }
    }
}
