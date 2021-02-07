//! # ArrayStack: Fast Stack Operations Using an Array
//!
//! See Chapter 2.1

use std::mem::MaybeUninit;
use std::ptr;

pub struct ArrayStack<T> {
    buf: Box<[MaybeUninit<T>]>,
    len: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Self {
            buf: [].into(),
            len: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            Some(unsafe { &*self.buf[i].as_ptr() })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.len {
            Some(unsafe { &mut *self.buf[i].as_mut_ptr() })
        } else {
            None
        }
    }

    pub fn add(&mut self, index: usize, x: T) {
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

        unsafe {
            let p = self.buf.as_mut_ptr().add(index);
            ptr::copy(p, p.offset(1), len - index);
            p.write(MaybeUninit::new(x));
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {}) should be < len (is {})", index, len);
        }
        let len = self.len;
        if index >= len {
            assert_failed(index, len);
        }
        let x;
        unsafe {
            let p = self.buf.as_mut_ptr().add(index);
            x = p.read().assume_init();
            ptr::copy(p.offset(1), p, len - index - 1);
        }
        self.len -= 1;
        if self.buf.len() >= 3 * self.len {
            self.resize();
        }
        x
    }

    fn resize(&mut self) {
        let len = self.len;
        let new_capacity = usize::max(1, 2 * len);
        let mut new_buffer = Vec::with_capacity(new_capacity);
        new_buffer.resize_with(new_capacity, MaybeUninit::uninit);
        let mut new_buffer = new_buffer.into_boxed_slice();
        unsafe {
            let src = self.buf.as_ptr();
            let dst = new_buffer.as_mut_ptr();
            ptr::copy_nonoverlapping(src, dst, len);
        }
        self.buf = new_buffer;
    }
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ArrayStack<T> {
    fn drop(&mut self) {
        for index in (0..self.len).rev() {
            // TODO: Prevent reallocation on drop.
            self.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayStack;

    #[test]
    fn should_add_elements() {
        let mut arr = ArrayStack::new();
        arr.add(0, "foo");
        arr.add(1, "bar");
        arr.add(2, "foobar");
        assert_eq!(arr.get(0), Some(&"foo"));
        assert_eq!(arr.get(1), Some(&"bar"));
        assert_eq!(arr.get(2), Some(&"foobar"));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn should_set_elements() {
        let mut arr = ArrayStack::new();
        arr.add(0, "foo");
        arr.add(1, "bar");
        arr.add(2, "foobar");
        *arr.get_mut(0).unwrap() = "my_foo";
        *arr.get_mut(1).unwrap() = "my_bar";
        *arr.get_mut(2).unwrap() = "my_foobar";
        assert_eq!(arr.get_mut(3), None);
        assert_eq!(arr.get(0), Some(&"my_foo"));
        assert_eq!(arr.get(1), Some(&"my_bar"));
        assert_eq!(arr.get(2), Some(&"my_foobar"));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn should_insert_elements() {
        let mut arr = ArrayStack::new();
        arr.add(0, "foo");
        arr.add(0, "bar");
        arr.add(0, "foobar");
        assert_eq!(arr.get(0), Some(&"foobar"));
        assert_eq!(arr.get(1), Some(&"bar"));
        assert_eq!(arr.get(2), Some(&"foo"));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn should_remove_elements() {
        let mut arr = ArrayStack::new();
        arr.add(0, "foo");
        arr.add(0, "bar");
        arr.add(0, "foobar");
        assert_eq!(arr.remove(0), "foobar");
        assert_eq!(arr.remove(0), "bar");
        assert_eq!(arr.remove(0), "foo");
        assert_eq!(arr.get(0), None);
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
            let mut arr = ArrayStack::new();
            arr.add(0, DropWatcher(&mut dropped));
        }
        assert!(dropped);
    }
}
