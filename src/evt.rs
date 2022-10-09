use crate::item::{Name, NamedItem};

pub enum Event {
    Success,

    /// Named item got.
    ItemGot(NamedItem),

    /// Nothing to get or nothing to remove.
    NoEntry(Name),

    /// Unable to push an item(storage or buffer full).
    TooManyItemsAlready,

    /// Unexpected error(unable to check item existence, ...).
    NoPerm(String),

    /// Specified name already exists(retry with next name).
    Again,

    /// Invalid request
    BadRequest,

    /// Item names got.
    NamesGot(Vec<Name>),

    /// Item got, but its contents broken(power failure?).
    Broken(Name),

    /// Item got, but unreadable(bit rot?).
    InvalidItem(String),
}
