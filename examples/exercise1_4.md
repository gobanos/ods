# Exercise 1.4.
Suppose you have a Stack, s, that supports only the push(x)
and pop() operations. Show how, using only a FIFO Queue, q, you can
reverse the order of all elements in s.

# Solution
```rust
fn reverse(s: &mut Stack) {
    let mut q = Queue::with_capacity(s.len());
    while let Some(x) = s.pop() {
        q.push(x);
    }
    while let Some(x) = q.pop() {
        s.push(x);
    }
}
```