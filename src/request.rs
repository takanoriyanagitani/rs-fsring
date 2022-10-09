use crate::item::{Item, Name};

pub enum Request {
    /// Get a named item.
    Get(Name),

    /// Remove a named item.
    Del(Name),

    /// Push an item.
    Push(Item),

    /// List names.
    List,
}
