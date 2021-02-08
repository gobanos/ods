//! A LIFO list container
//!
//! See `Stack`

/// A LIFO list container
pub trait Stack {
    type Item;

    /// Insert a new item on top of the stack
    fn push(&mut self, item: Self::Item);

    /// Remove and return the item on top of the stack
    fn pop(&mut self) -> Option<Self::Item>;
}
