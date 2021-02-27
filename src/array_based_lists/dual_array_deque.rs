//! # DualArrayDeque: Building a Deque from Two Stacks
//!
//! See Chapter 2.5

use crate::array_based_lists::array_stack::ArrayStack;

/// DualArrayDeque: Building a Deque from Two Stacks
pub struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T> DualArrayDeque<T> {
    pub fn new() -> Self {
        DualArrayDeque {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

impl<T> Default for DualArrayDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}
