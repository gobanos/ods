//! # ArrayQueue: An Array-Based Queue
//!
//! See Chapter 2.3

use crate::queue::Queue;
use std::mem::MaybeUninit;
use std::ptr;

/// ArrayQueue: An Array-Based Queue
pub struct ArrayQueue<T> {
    buf: Box<[MaybeUninit<T>]>,
    len: usize,
    offset: usize,
}

impl<T> ArrayQueue<T> {
    pub fn new() -> Self {
        ArrayQueue {
            buf: [].into(),
            len: 0,
            offset: 0,
        }
    }

    pub fn add(&mut self, element: T) {
        let len = self.len;
        if len + 1 > self.buf.len() {
            self.resize();
        }
        unsafe {
            *self
                .buf
                .get_unchecked_mut((len + self.offset) % self.buf.len()) =
                MaybeUninit::new(element);
        }
        self.len += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        let len = self.len;
        if len > 0 {
            let element;
            unsafe {
                let p = self.buf.as_mut_ptr().add(self.offset);
                element = p.read().assume_init();
            }
            self.offset = (self.offset + 1) % self.buf.len();
            self.len -= 1;
            if self.buf.len() >= 3 * self.len {
                self.resize();
            }
            Some(element)
        } else {
            None
        }
    }

    fn resize(&mut self) {
        let len = self.len;
        let offset = self.offset;
        let old_capacity = self.buf.len();
        let new_capacity = usize::max(1, 2 * len);
        let mut new_buffer = Vec::with_capacity(new_capacity);
        new_buffer.resize_with(new_capacity, MaybeUninit::uninit);
        let mut new_buffer = new_buffer.into_boxed_slice();
        unsafe {
            if offset + len > old_capacity {
                // Copy offset..old_capacity
                let src = self.buf.as_ptr().add(self.offset);
                let dst = new_buffer.as_mut_ptr();
                ptr::copy_nonoverlapping(src, dst, old_capacity - offset);
                // Copy 0..(offset + len - old_capacity)
                let src = self.buf.as_ptr();
                let dst = new_buffer.as_mut_ptr().add(old_capacity - offset);
                ptr::copy_nonoverlapping(src, dst, offset + len - old_capacity);
            } else {
                // Copy offset..offset + len
                let src = self.buf.as_ptr().add(self.offset);
                let dst = new_buffer.as_mut_ptr();
                ptr::copy_nonoverlapping(src, dst, len);
            }
        }
        self.buf = new_buffer;
        self.offset = 0;
    }
}

impl<T> Default for ArrayQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ArrayQueue<T> {
    fn drop(&mut self) {
        loop {
            if self.remove().is_none() {
                break;
            }
        }
    }
}

impl<T> Queue for ArrayQueue<T> {
    type Item = T;

    fn add(&mut self, item: Self::Item) {
        ArrayQueue::add(self, item)
    }

    fn remove(&mut self) -> Option<Self::Item> {
        ArrayQueue::remove(self)
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue;
    use std::collections::VecDeque;

    #[test]
    fn should_add_and_remove_elements() {
        let mut arr = ArrayQueue::new();
        arr.add("foo");
        arr.add("bar");
        arr.add("foobar");
        assert_eq!(arr.remove(), Some("foo"));
        assert_eq!(arr.remove(), Some("bar"));
        assert_eq!(arr.remove(), Some("foobar"));
        assert_eq!(arr.remove(), None);
    }

    #[test]
    fn should_reallocate_properly() {
        // Compare to reference impl: VecDeque used as a FIFO
        let mut arr = ArrayQueue::new();
        let mut deque = VecDeque::new();
        const LEN: i32 = 100;
        for i in 0..LEN {
            arr.add(i);
            deque.push_back(i);
            for j in 0..i {
                arr.add(j);
                deque.push_back(j);
            }
            for _ in 0..i {
                assert_eq!(arr.remove(), deque.pop_front());
            }
        }
        for _ in 0..LEN {
            assert_eq!(arr.remove(), deque.pop_front());
        }
        assert_eq!(arr.remove(), None);
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn should_drop_elements() {
        struct DropWatcher<'a>(&'a mut bool);
        impl Drop for DropWatcher<'_> {
            fn drop(&mut self) {
                *self.0 = true;
            }
        }
        let mut dropped = false;
        {
            let mut arr = ArrayQueue::new();
            arr.add(DropWatcher(&mut dropped));
        }
        assert!(dropped);
    }
}
