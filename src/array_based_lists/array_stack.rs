//! # ArrayStack: Fast Stack Operations Using an Array
//!
//! See Chapter 2.1

use std::mem;

pub struct ArrayStack<T> {
    a: Vec<T>,
    n: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Self {
            a: Vec::new(),
            n: 0,
        }
    }

    pub fn get(&self, i: usize) -> &T {
        &self.a[i]
    }

    pub fn set(&mut self, i: usize, x: T) -> T {
        mem::replace(&mut self.a[i], x)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.n == self.a.len() {
            self.resize();
        }
        self.a[self.n] = x;
        for j in (i..self.n).rev() {
            self.a.swap(j, j + 1);
        }

        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        for j in i..self.n {
            self.a.swap(j, j + 1);
        }
        let x = self.a.pop().expect("the underlying array is empty");
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }

    fn resize(&mut self) {
        todo!()
    }
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
