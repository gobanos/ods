//! A FIFO list container
//!
//! See `Queue`

/// A FIFO list container
pub trait Queue {
    type Item;

    /// Insert a new item at the end of the queue
    fn add(&mut self, item: Self::Item);

    /// Remove and return the item in front of the queue
    fn remove(&mut self) -> Option<Self::Item>;
}
