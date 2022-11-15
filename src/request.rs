use crate::item::{Item, Name};

/// A list of supported operations.
#[non_exhaustive]
pub enum Request {
    /// Get a named item.
    Get(Name),

    /// Remove a named item.
    Del(Name),

    /// Push an item.
    Push(Item),

    /// List names.
    List,

    /// Remove broken items.
    Vacuum,
}
