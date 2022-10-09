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

impl Request {
    pub fn new_get(n: Name) -> Self {
        Self::Get(n)
    }

    pub fn new_del(n: Name) -> Self {
        Self::Del(n)
    }

    pub fn new_push(i: Item) -> Self {
        Self::Push(i)
    }

    pub fn new_list() -> Self {
        Self::List
    }
}
