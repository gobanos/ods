//! # ArrayDeque: Fast Deque Operations Using an Array
//!
//! See Chapter 2.4

use std::mem::MaybeUninit;
use std::ptr;

/// ArrayDeque: Fast Deque Operations Using an Array
pub struct ArrayDeque<T> {
    buf: Box<[MaybeUninit<T>]>,
    len: usize,
    offset: usize,
}

impl<T> ArrayDeque<T> {
    pub fn new() -> Self {
        ArrayDeque {
            buf: [].into(),
            len: 0,
            offset: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let len = self.len;
        if index < len {
            Some(unsafe { &*self.buf[(index + self.offset) % self.buf.len()].as_ptr() })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let len = self.len;
        if index < len {
            Some(unsafe { &mut *self.buf[(index + self.offset) % self.buf.len()].as_mut_ptr() })
        } else {
            None
        }
    }

    pub fn add(&mut self, index: usize, element: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
        let len = self.len;
        if index > len {
            assert_failed(index, len);
        }

        if len == self.buf.len() {
            self.resize();
        }
        let capacity = self.buf.len();

        let p = self.buf.as_mut_ptr();
        if index < len / 2 {
            // shift to the left 0..index
            self.offset = self.offset.checked_sub(1).unwrap_or(capacity - 1);
            for k in 0..index {
                unsafe {
                    ptr::write(
                        p.add((self.offset + k) % capacity),
                        ptr::read(p.add((self.offset + k + 1) % capacity)),
                    );
                }
            }
        } else {
            // shift to the right index..len
            for k in (index..len).rev() {
                unsafe {
                    ptr::write(
                        p.add((self.offset + k + 1) % capacity),
                        ptr::read(p.add((self.offset + k) % capacity)),
                    );
                }
            }
        }
        unsafe {
            ptr::write(
                p.add((self.offset + index) % capacity),
                MaybeUninit::new(element),
            );
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let len = self.len;
        if index >= len {
            return None;
        }

        let capacity = self.buf.len();
        let p = self.buf.as_mut_ptr();
        let element =
            unsafe { Some(ptr::read(p.add((self.offset + index) % capacity)).assume_init()) };
        if index < len / 2 {
            // shift to the right 0..index
            for k in (0..index).rev() {
                unsafe {
                    ptr::write(
                        p.add((self.offset + k + 1) % capacity),
                        ptr::read(p.add((self.offset + k) % capacity)),
                    );
                }
            }
            self.offset = (self.offset + 1) % capacity;
        } else {
            // shift to the left index + 1..len
            for k in index + 1..len {
                unsafe {
                    ptr::write(
                        p.add((self.offset + k) % capacity),
                        ptr::read(p.add((self.offset + k + 1) % capacity)),
                    );
                }
            }
        }

        self.len -= 1;
        if capacity >= 3 * self.len {
            self.resize();
        }
        element
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

impl<T> Default for ArrayDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ArrayDeque<T> {
    fn drop(&mut self) {
        for i in (0..self.len).rev() {
            self.remove(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayDeque;
    use std::collections::VecDeque;

    #[test]
    fn should_add_and_remove_elements() {
        let mut arr = ArrayDeque::new();
        arr.add(0, "foo");
        arr.add(1, "bar");
        arr.add(1, "foobar");
        assert_eq!(arr.remove(0), Some("foo"));
        assert_eq!(arr.remove(0), Some("foobar"));
        assert_eq!(arr.remove(0), Some("bar"));
        assert_eq!(arr.remove(0), None);
    }

    #[test]
    fn should_reallocate_properly() {
        // Compare to reference impl: VecDeque used as a FIFO
        let mut arr = ArrayDeque::new();
        let mut deque = VecDeque::new();
        const LEN: usize = 100;
        for i in 0..LEN {
            arr.add(i, i);
            deque.insert(i, i);
            for j in 0..i {
                arr.add(j, j);
                deque.insert(j, j);
            }
            for j in (0..i).rev() {
                assert_eq!(arr.remove(j), deque.remove(j));
            }
        }
        for _ in 0..LEN {
            assert_eq!(arr.remove(0), deque.remove(0));
        }
        assert_eq!(arr.remove(0), None);
        assert_eq!(deque.remove(0), None);
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
            let mut arr = ArrayDeque::new();
            arr.add(0, DropWatcher(&mut dropped));
        }
        assert!(dropped);
    }
}
